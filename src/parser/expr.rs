use std::any::Any;

use crate::lexer::TokenType;

#[derive(Debug)]
pub struct Program {
    pub expr: Expr,
}


#[derive(Debug)]
pub enum Literal {
    Integer(i32),
    FloatingPoint(f64),
    String(String),
    Boolean(bool),
    Terminal(Terminal)
}

#[derive(Debug)]
pub struct Terminal {
    pub value: Box<dyn Any>
}


#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expr>
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: TokenType,
    pub right: Box<Expr>,
}


#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: TokenType,
    pub right: Box<Expr>
}

#[derive(Debug)]
pub struct AssignExpr {
    pub name: TokenType,
    pub value: Box<Expr>
}

#[derive(Debug)]
pub struct UpdateExpr {
    pub name: TokenType,
    pub op: TokenType,
    pub change: Box<Expr>
}

#[derive(Debug)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: TokenType,
    pub args: Vec<Expr>
}


#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    Grouping(Grouping),
    Literal(Literal),
    Variable(TokenType),
    Assign(AssignExpr),
    Update(UpdateExpr),
    Call(Call)
}

