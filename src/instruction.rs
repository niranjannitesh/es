use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Register {
    pub index: usize,
}

#[derive(Clone, Debug)]
pub enum Instruction {
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
