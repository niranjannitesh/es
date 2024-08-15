use std::collections::HashMap;

use crate::{instruction::Instruction, value::Value, vm::VMError};

pub struct Process {
    pid: usize,
    registers: Vec<Value>,
    ip: usize,
    program: Vec<Instruction>,
    pub variables: HashMap<String, Value>,
    halt: bool,
}

impl Process {
    pub fn new(pid: usize) -> Self {
        Process {
            pid,
            registers: Vec::new(),
            program: Vec::new(),
            variables: HashMap::new(),
            ip: 0,
            halt: false,
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>, max_registers: usize) {
        self.program = program;
        self.registers = vec![Value::Empty; max_registers];
    }

    pub fn run_program(&mut self) -> Result<(), VMError> {
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
                let value1 = &self.registers[reg1.index];
                let value2 = &self.registers[reg2.index];
                match (value1, value2) {
                    (Value::Number(v1), Value::Number(v2)) => {
                        self.registers[dest.index] = Value::Number(v1 + v2);
                    }
                    (Value::String(s1), Value::String(s2)) => {
                        self.registers[dest.index] = Value::String(s1.clone() + s2);
                    }
                    (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
                        self.registers[dest.index] = Value::String(s.clone() + &n.to_string());
                    }
                    _ => return Err(VMError::TypeMisMatch(self.pid)),
                }
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
