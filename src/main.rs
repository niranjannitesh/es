mod ast;
mod generator;
mod instruction;
mod process;
mod value;
mod vm;

use ast::{ASTNode, BinaryOperator};
use generator::BytecodeGenerator;
use instruction::Instruction;
use vm::ByteCodeVM;

fn main() {
    let mut vm = ByteCodeVM::new();
    let process = vm.spawn();

    let ast = ASTNode::Block(vec![
        ASTNode::Assignment("x".to_string(), Box::new(ASTNode::NumberLiteral(0.0))),
        ASTNode::While(
            Box::new(ASTNode::BinaryOp(
                Box::new(ASTNode::NumberLiteral(5.0)),
                BinaryOperator::Subtract,
                Box::new(ASTNode::Variable("x".to_string())),
            )),
            Box::new(ASTNode::Block(vec![
                ASTNode::Assignment(
                    "x".to_string(),
                    Box::new(ASTNode::BinaryOp(
                        Box::new(ASTNode::Variable("x".to_string())),
                        BinaryOperator::Add,
                        Box::new(ASTNode::NumberLiteral(1.0)),
                    )),
                ),
                ASTNode::Assignment(
                    "temp".to_string(),
                    Box::new(ASTNode::BinaryOp(
                        Box::new(ASTNode::NumberLiteral(5.0)),
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
                Box::new(ASTNode::NumberLiteral(5.0)),
            )),
            Box::new(ASTNode::Assignment(
                "result".to_string(),
                Box::new(ASTNode::NumberLiteral(1.0)),
            )),
            Some(Box::new(ASTNode::Assignment(
                "result".to_string(),
                Box::new(ASTNode::NumberLiteral(0.0)),
            ))),
        ),
        ASTNode::Assignment(
            "hello".to_string(),
            Box::new(ASTNode::StringLiteral("hello ".to_string())),
        ),
        ASTNode::Assignment("world".to_string(), Box::new(ASTNode::NumberLiteral(38.0))),
        ASTNode::Assignment(
            "str".to_string(),
            Box::new(ASTNode::BinaryOp(
                Box::new(ASTNode::Variable("hello".to_string())),
                BinaryOperator::Add,
                Box::new(ASTNode::Variable("world".to_string())),
            )),
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
    generator
        .instructions
        .push(Instruction::DbgPrintVar("str".to_string()));

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
