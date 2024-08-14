use std::{
    collections::HashMap,
    fmt::{self, Debug, Display},
};

#[derive(Debug)]
enum VMError {
    TypeMisMatch(usize),
    DivisionByZero(usize),
    BadAddress(usize),
    UndefinedVariable(usize, String),
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::DivisionByZero(pid) => write!(f, "[Process #{}] division by zero", pid),
            VMError::TypeMisMatch(pid) => write!(f, "[Process #{}] type mistmatch", pid),
            VMError::BadAddress(pid) => write!(f, "[process #{}] bad address", pid),
            VMError::UndefinedVariable(pid, name) => {
                write!(f, "[process #{}] undefined variable `{}`", pid, name)
            }
        }
    }
}

enum ASTNode {
    Number(f64),
    BinaryOp(Box<ASTNode>, BinaryOperator, Box<ASTNode>),
    Variable(String),
    Assignment(String, Box<ASTNode>),
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    While(Box<ASTNode>, Box<ASTNode>),
    Block(Vec<ASTNode>),
}

enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct BytecodeGenerator {
    instructions: Vec<Instruction>,
    next_register: usize,
    variables: HashMap<String, Register>,
}

impl BytecodeGenerator {
    fn new() -> Self {
        BytecodeGenerator {
            instructions: Vec::new(),
            next_register: 0,
            variables: HashMap::new(),
        }
    }

    fn allocate_register(&mut self) -> Register {
        let reg = Register {
            index: self.next_register,
        };
        self.next_register += 1;
        reg
    }

    fn generate(&mut self, node: &ASTNode) -> Register {
        match node {
            ASTNode::Number(value) => {
                let reg = self.allocate_register();
                self.instructions
                    .push(Instruction::Load(reg.clone(), Value::Number(*value)));
                reg
            }
            ASTNode::BinaryOp(left, op, right) => {
                let left_reg = self.generate(left);
                let right_reg = self.generate(right);
                let result_reg = self.allocate_register();
                let instruction = match op {
                    BinaryOperator::Add => {
                        Instruction::Add(result_reg.clone(), left_reg, right_reg)
                    }
                    BinaryOperator::Subtract => {
                        Instruction::Sub(result_reg.clone(), left_reg, right_reg)
                    }
                    BinaryOperator::Multiply => {
                        Instruction::Mul(result_reg.clone(), left_reg, right_reg)
                    }
                    BinaryOperator::Divide => {
                        Instruction::Div(result_reg.clone(), left_reg, right_reg)
                    }
                };
                self.instructions.push(instruction);
                result_reg
            }
            ASTNode::Variable(name) => {
                let reg = self.allocate_register();
                self.instructions
                    .push(Instruction::LoadVar(reg.clone(), name.clone()));
                reg
            }
            ASTNode::Assignment(name, value) => {
                let value_reg = self.generate(value);
                self.instructions
                    .push(Instruction::Store(name.clone(), value_reg.clone()));
                self.variables.insert(name.clone(), value_reg.clone());
                value_reg
            }
            ASTNode::Block(statements) => {
                let mut last_reg = self.allocate_register();
                for statement in statements {
                    last_reg = self.generate(statement);
                }
                last_reg
            }
            ASTNode::If(condition, then_branch, else_branch) => {
                let condition_reg = self.generate(condition);
                let then_label = self.instructions.len();
                self.instructions
                    .push(Instruction::JmpFalse(condition_reg.clone(), 0));
                self.generate(then_branch);
                let end_label = self.instructions.len();
                if let Some(else_branch) = else_branch {
                    let else_label = self.instructions.len();
                    self.instructions.push(Instruction::Jmp(0));
                    self.instructions[then_label] =
                        Instruction::JmpFalse(condition_reg.clone(), else_label + 1);
                    self.generate(else_branch);
                    self.instructions[else_label] = Instruction::Jmp(self.instructions.len());
                } else {
                    self.instructions[then_label] = Instruction::JmpFalse(condition_reg, end_label);
                }
                self.allocate_register()
            }
            ASTNode::While(condition, body) => {
                let loop_start = self.instructions.len();
                let condition_reg = self.generate(condition);
                let body_start = self.instructions.len();
                self.instructions
                    .push(Instruction::JmpFalse(condition_reg.clone(), 0)); // reassignment_below
                self.generate(body);
                self.instructions.push(Instruction::Jmp(loop_start));
                let loop_end = self.instructions.len();
                self.instructions[body_start] = Instruction::JmpFalse(condition_reg, loop_end);
                self.allocate_register()
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Value {
    Empty,
    Number(f64),
    Boolean(bool),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Empty => write!(f, "[empty]"),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Number(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Debug)]
struct Register {
    index: usize,
}

#[derive(Clone, Debug)]
enum Instruction {
    Halt,
    Load(Register, Value),
    Store(String, Register),
    LoadVar(Register, String),
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Mul(Register, Register, Register),
    Div(Register, Register, Register),
    Jmp(usize),
    JmpFalse(Register, usize),
    DbgPrintReg(Register),
    DbgPrintVar(String),
}

struct Process {
    pid: usize,
    registers: Vec<Value>,
    ip: usize,
    program: Vec<Instruction>,
    variables: HashMap<String, Value>,
    halt: bool,
}

impl Process {
    fn new(pid: usize) -> Self {
        Process {
            pid,
            registers: Vec::new(),
            program: Vec::new(),
            variables: HashMap::new(),
            ip: 0,
            halt: false,
        }
    }

    fn load_program(&mut self, program: Vec<Instruction>, max_registers: usize) {
        self.program = program;
        self.registers = vec![Value::Empty; max_registers];
    }

    fn run_program(&mut self) -> Result<(), VMError> {
        self.ip = 0;
        while self.ip < self.program.len() && !self.halt {
            let instruction = self.program[self.ip].clone();
            match self.execute(instruction) {
                Ok(_) => {
                    // self.dump();
                    // dbg!("{}", &self.variables);
                }
                Err(e) => return Err(e),
            }
            self.ip += 1;
        }
        Ok(())
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), VMError> {
        match instruction {
            Instruction::Halt => {
                self.halt = true;
            }
            Instruction::Load(reg, value) => {
                self.registers[reg.index] = value;
            }
            Instruction::Add(dest, reg1, reg2) => {
                let value1 = match self.registers[reg1.index] {
                    Value::Number(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                let value2 = match self.registers[reg2.index] {
                    Value::Number(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                self.registers[dest.index] = Value::Number(value1 + value2);
            }
            Instruction::Sub(dest, reg1, reg2) => {
                let value1 = match self.registers[reg1.index] {
                    Value::Number(val) => val,
                    _ => {
                        return Err(VMError::TypeMisMatch(self.pid));
                    }
                };
                let value2 = match self.registers[reg2.index] {
                    Value::Number(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                self.registers[dest.index] = Value::Number(value1 - value2);
            }
            Instruction::Mul(dest, reg1, reg2) => {
                let value1 = match self.registers[reg1.index] {
                    Value::Number(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                let value2 = match self.registers[reg2.index] {
                    Value::Number(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                self.registers[dest.index] = Value::Number(value1 * value2);
            }
            Instruction::Div(dest, reg1, reg2) => {
                let value1 = match self.registers[reg1.index] {
                    Value::Number(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                let value2 = match self.registers[reg2.index] {
                    Value::Number(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                self.registers[dest.index] = Value::Number(value1 / value2);
            }
            Instruction::Jmp(dest) => {
                self.ip = dest;
            }
            Instruction::JmpFalse(reg, dest) => {
                let cond = match self.registers[reg.index] {
                    Value::Number(val) => val > 0.0,
                    Value::Boolean(val) => val,
                    _ => false,
                };
                if !cond {
                    self.ip = dest - 1;
                }
            }
            Instruction::Store(var_name, reg) => {
                let value = self.registers[reg.index].clone();
                self.variables.insert(var_name, value);
            }
            Instruction::LoadVar(reg, var_name) => {
                if let Some(value) = self.variables.get(&var_name) {
                    self.registers[reg.index] = value.clone();
                } else {
                    return Err(VMError::UndefinedVariable(self.pid, var_name));
                }
            }
            Instruction::DbgPrintReg(reg) => {
                println!(
                    "[Process #{}] r{}: {}",
                    self.pid, reg.index, &self.registers[reg.index]
                );
            }
            Instruction::DbgPrintVar(name) => {
                println!(
                    "[Process #{}] {}: {}",
                    self.pid,
                    name.clone(),
                    self.variables.get(&name).unwrap()
                );
            }
        }
        Ok(())
    }

    fn dump(&mut self) {
        println!("[Process Stack #{}]", self.pid);
        for val in &self.registers {
            match val {
                Value::Number(x) => {
                    println!("\t{}", x);
                }
                Value::Boolean(x) => {
                    println!("\t{}", x);
                }
                Value::String(x) => {
                    println!("\t{}", x);
                }

                Value::Empty => {
                    println!("\t[empty]");
                }
            }
        }
    }
}

struct ByteCodeVM {
    processes: Vec<Process>,
}

impl ByteCodeVM {
    fn new() -> Self {
        ByteCodeVM {
            processes: Vec::new(),
        }
    }

    fn spawn(&mut self) -> &mut Process {
        let process = Process::new(self.processes.len());
        self.processes.push(process);
        self.processes.last_mut().unwrap()
    }
}

fn main() {
    let mut vm = ByteCodeVM::new();
    let process = vm.spawn();

    let ast = ASTNode::Block(vec![
        ASTNode::Assignment("x".to_string(), Box::new(ASTNode::Number(0.0))),
        ASTNode::While(
            Box::new(ASTNode::BinaryOp(
                Box::new(ASTNode::Number(5.0)),
                BinaryOperator::Subtract,
                Box::new(ASTNode::Variable("x".to_string())),
            )),
            Box::new(ASTNode::Block(vec![
                ASTNode::Assignment(
                    "x".to_string(),
                    Box::new(ASTNode::BinaryOp(
                        Box::new(ASTNode::Variable("x".to_string())),
                        BinaryOperator::Add,
                        Box::new(ASTNode::Number(1.0)),
                    )),
                ),
                ASTNode::Assignment(
                    "temp".to_string(),
                    Box::new(ASTNode::BinaryOp(
                        Box::new(ASTNode::Number(5.0)),
                        BinaryOperator::Subtract,
                        Box::new(ASTNode::Variable("x".to_string())),
                    )),
                ),
            ])),
        ),
        ASTNode::If(
            Box::new(ASTNode::BinaryOp(
                Box::new(ASTNode::Variable("x".to_string())),
                BinaryOperator::Subtract,
                Box::new(ASTNode::Number(5.0)),
            )),
            Box::new(ASTNode::Assignment(
                "result".to_string(),
                Box::new(ASTNode::Number(1.0)),
            )),
            Some(Box::new(ASTNode::Assignment(
                "result".to_string(),
                Box::new(ASTNode::Number(0.0)),
            ))),
        ),
    ]);

    let mut generator = BytecodeGenerator::new();
    generator.generate(&ast);

    generator
        .instructions
        .push(Instruction::DbgPrintVar("x".to_string()));
    generator
        .instructions
        .push(Instruction::DbgPrintVar("result".to_string()));

    process.load_program(generator.instructions, generator.next_register);
    match process.run_program() {
        Ok(_) => {
            println!("variable states:");
            for (name, value) in &process.variables {
                println!("\t {}: {}", name, value);
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}
