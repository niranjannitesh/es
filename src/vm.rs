use std::fmt;

use crate::process::Process;

#[derive(Debug)]
pub enum VMError {
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

pub struct ByteCodeVM {
    processes: Vec<Process>,
}

impl ByteCodeVM {
    pub fn new() -> Self {
        ByteCodeVM {
            processes: Vec::new(),
        }
    }

    pub fn spawn(&mut self) -> &mut Process {
        let process = Process::new(self.processes.len());
        self.processes.push(process);
        self.processes.last_mut().unwrap()
    }
}
