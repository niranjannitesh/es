use std::collections::HashMap;

use crate::{
    ast::{ASTNode, BinaryOperator},
    instruction::{Instruction, Register},
    value::Value,
};

pub struct BytecodeGenerator {
    pub instructions: Vec<Instruction>,
    pub next_register: usize,
    pub variables: HashMap<String, Register>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
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

    pub fn generate(&mut self, node: &ASTNode) -> Register {
        match node {
            ASTNode::NumberLiteral(value) => {
                let reg = self.allocate_register();
                self.instructions
                    .push(Instruction::Load(reg.clone(), Value::Number(*value)));
                reg
            }
            ASTNode::StringLiteral(string) => {
                let reg = self.allocate_register();
                self.instructions.push(Instruction::Load(
                    reg.clone(),
                    Value::String(string.clone()),
                ));
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
