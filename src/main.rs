use std::fmt;

#[derive(Debug)]
enum VMError {
    TypeMisMatch(usize),
    DivisionByZero(usize),
    BadAddress(usize),
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::DivisionByZero(pid) => write!(f, "[Process #{}] Division by zero", pid),
            VMError::TypeMisMatch(pid) => write!(f, "[Process #{}] Type mistmatch", pid),
            VMError::BadAddress(pid) => write!(f, "[Process #{}] Bad Address", pid),
        }
    }
}

#[derive(Clone, Debug)]
enum Value {
    Empty,
    Default(u64),
}

#[derive(Clone, Debug)]
struct Register {
    index: usize,
}

#[derive(Clone, Debug)]
enum Instruction {
    Load(Register, Value),
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
    halt: bool,
}

impl Process {
    fn new(pid: usize) -> Self {
        Process {
            pid,
            registers: Vec::new(),
            program: Vec::new(),
            ip: 0,
            halt: false,
        }
    }

    fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
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
            println!("{:?}", instruction);
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
            Instruction::Load(reg, value) => {
                self.registers[reg.index] = value;
            }
            Instruction::Add(dest, reg1, reg2) => {
                let value1 = match self.registers[reg1.index] {
                    Value::Default(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                let value2 = match self.registers[reg2.index] {
                    Value::Default(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                self.registers[dest.index] = Value::Default(value1.wrapping_add(value2));
            }
            Instruction::Sub(dest, reg1, reg2) => {
                let value1 = match self.registers[reg1.index] {
                    Value::Default(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                let value2 = match self.registers[reg2.index] {
                    Value::Default(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                self.registers[dest.index] = Value::Default(value1.wrapping_sub(value2));
            }
            Instruction::Mul(dest, reg1, reg2) => {
                let value1 = match self.registers[reg1.index] {
                    Value::Default(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                let value2 = match self.registers[reg2.index] {
                    Value::Default(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                self.registers[dest.index] = Value::Default(value1.wrapping_mul(value2));
            }
            Instruction::Div(dest, reg1, reg2) => {
                let value1 = match self.registers[reg1.index] {
                    Value::Default(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                let value2 = match self.registers[reg2.index] {
                    Value::Default(val) => val,
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                };
                if value2 == 0 {
                    return Err(VMError::DivisionByZero(self.pid));
                }
                self.registers[dest.index] = Value::Default(value1.wrapping_div(value2));
            }
            Instruction::Jmp(dest) => {
                self.ip = match self.registers[dest.index] {
                    Value::Default(x) => x as usize,
                    _ => return Err(VMError::BadAddress(self.pid)),
                };
            }
            Instruction::JmpTrue(reg, dest) => {
                let cond = match self.registers[reg.index] {
                    Value::Default(val) => val > 0,
                    _ => false,
                };
                if cond {
                    self.ip = match self.registers[dest.index] {
                        Value::Default(x) => x as usize,
                        _ => return Err(VMError::BadAddress(self.pid)),
                    };
                } else {
                    self.ip += 1;
                }
            }
            Instruction::JmpFalse(reg, dest) => {
                let cond = match self.registers[reg.index] {
                    Value::Default(val) => val > 0,
                    _ => false,
                };
                if !cond {
                    self.ip = match self.registers[dest.index] {
                        Value::Default(x) => x as usize,
                        _ => return Err(VMError::BadAddress(self.pid)),
                    };
                } else {
                    self.ip += 1;
                }
            }
        }
        Ok(())
    }

    fn dump(&mut self) {
        println!("[Process #{}]", self.pid);
        for val in &self.registers {
            match val {
                Value::Default(val) => {
                    println!("\t{}", val);
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
    let reg1 = process.allocate_reg();
    let reg2 = process.allocate_reg();
    let reg3 = process.allocate_reg();
    let reg4 = process.allocate_reg();

    let program = vec![
        Instruction::Load(reg1.clone(), Value::Default(50)),
        Instruction::Load(reg2.clone(), Value::Default(20)),
        Instruction::Add(reg3.clone(), reg1.clone(), reg2.clone()),
        Instruction::Sub(reg3.clone(), reg1.clone(), reg2.clone()),
        Instruction::Mul(reg3.clone(), reg1.clone(), reg2.clone()),
        Instruction::Div(reg3.clone(), reg1.clone(), reg2.clone()),
        Instruction::Load(reg4.clone(), Value::Default(0)),
        Instruction::Jmp(reg4),
    ];

    process.load_program(program);
    match process.run_program() {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}
