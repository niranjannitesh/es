type Double = f64;

enum Value {
    Number(Double),
}

#[derive(Clone)]
struct Register {
    index: usize,
}

enum Instruction {
    Load(Register, Value),
    Add(Register, Register, Register),
}

type Pid = usize;

struct Process {
    pid: Pid,
    registers: Vec<Value>,
    ip: usize,
}

impl Process {
    fn new(pid: usize) -> Self {
        Process {
            pid: pid,
            registers: Vec::new(),
            ip: 0,
        }
    }

    fn allocate_reg(&mut self) -> Register {
        self.registers.push(Value::Number(0.0));
        Register {
            index: self.registers.len() - 1,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Load(reg, value) => {
                self.registers[reg.index] = value;
            }
            Instruction::Add(dest, reg1, reg2) => {
                let mut value1 = match self.registers[reg1.index] {
                    Value::Number(val) => val,
                };
                let mut value2 = match self.registers[reg2.index] {
                    Value::Number(val) => val,
                };
                self.registers[dest.index] = Value::Number(value1 + value2);
            }
        }
    }

    fn dump(&mut self) {
        println!("[Process #{}]", self.pid);
        for val in &self.registers {
            match val {
                Value::Number(val) => {
                    println!("\t{}", val);
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
        let mut process = Process::new(self.processes.len());
        self.processes.push(process);
        self.processes.last_mut().unwrap()
    }
}

fn main() {
    let mut vm = ByteCodeVM::new();
    let mut process = vm.spawn();
    let reg1 = process.allocate_reg();
    let reg2 = process.allocate_reg();
    let reg3 = process.allocate_reg();

    process.execute(Instruction::Load(reg1.clone(), Value::Number(10.0)));
    process.execute(Instruction::Load(reg2.clone(), (Value::Number(20.0))));
    process.execute(Instruction::Add(reg3, reg1.clone(), reg2.clone()));
    process.dump();
}
