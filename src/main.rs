use std::{collections::HashMap, fmt};

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
    Jmp(Register),
    JmpTrue(Register, Register),
    JmpFalse(Register, Register),
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

    fn allocate_reg(&mut self) -> Register {
        self.registers.push(Value::Empty);
        Register {
            index: self.registers.len() - 1,
        }
    }

    fn run_program(&mut self) -> Result<(), VMError> {
        while self.ip < self.program.len() && !self.halt {
            let instruction = self.program[self.ip].clone();
            match self.execute(instruction) {
                Ok(_) => self.dump(),
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
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
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
                self.ip = match self.registers[dest.index] {
                    Value::Number(x) => x as usize,
                    _ => return Err(VMError::BadAddress(self.pid)),
                };
            }
            Instruction::JmpTrue(reg, dest) => {
                let cond = match self.registers[reg.index] {
                    Value::Number(val) => val > 0.0,
                    _ => false,
                };
                if cond {
                    self.ip = match self.registers[dest.index] {
                        Value::Number(x) => x as usize,
                        _ => return Err(VMError::BadAddress(self.pid)),
                    };
                } else {
                    self.ip += 1;
                }
            }
            Instruction::JmpFalse(reg, dest) => {
                let cond = match self.registers[reg.index] {
                    Value::Number(val) => val > 0.0,
                    _ => false,
                };
                if !cond {
                    self.ip = match self.registers[dest.index] {
                        Value::Number(x) => x as usize,
                        _ => return Err(VMError::BadAddress(self.pid)),
                    };
                } else {
                    self.ip += 1;
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

    let ast = ASTNode::BinaryOp(
        Box::new(ASTNode::Assignment(
            "z".to_string(),
            Box::new(ASTNode::BinaryOp(
                Box::new(ASTNode::Assignment(
                    "x".to_string(),
                    Box::new(ASTNode::Number(20.0)),
                )),
                BinaryOperator::Add,
                Box::new(ASTNode::Assignment(
                    "y".to_string(),
                    Box::new(ASTNode::Number(80.0)),
                )),
            )),
        )),
        BinaryOperator::Multiply,
        Box::new(ASTNode::Number(2.0)),
    );

    let mut generator = BytecodeGenerator::new();
    generator.generate(&ast);

    process.load_program(generator.instructions, generator.next_register);
    match process.run_program() {
        Ok(_) => {
            dbg!("variables: {}", generator.next_register);
        }
        Err(err) => eprintln!("{}", err),
    }
}
