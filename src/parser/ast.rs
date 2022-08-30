use std::any::Any;

use crate::lexer::TokenType;

#[derive(Debug)]
pub struct Program {
    pub expr: Expr,
}

impl Program {}


#[derive(Debug)]
pub enum Literal {
    Integer(i32),
    FloatingPoint(f64),
    String(String),
    Boolean(bool),
    Terminal(Terminal)
}


impl Literal {}


#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expr>
}

impl Grouping {}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: TokenType,
    pub right: Box<Expr>,
}

impl UnaryExpr {}


#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: TokenType,
    pub right: Box<Expr>
}

impl BinaryExpr {}


#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    Grouping(Grouping),
    Literal(Literal)
}

impl Expr {}


#[derive(Debug)]
pub struct Terminal {
    pub value: Box<dyn Any>
}