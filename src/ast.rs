pub enum ASTNode {
    StringLiteral(String),
    NumberLiteral(f64),
    BinaryOp(Box<ASTNode>, BinaryOperator, Box<ASTNode>),
    Variable(String),
    Assignment(String, Box<ASTNode>),
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    While(Box<ASTNode>, Box<ASTNode>),
    Block(Vec<ASTNode>),
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
