// pub trait Ast : std::fmt::Debug {
//
// }
//
// #[derive(Debug)]


use crate::lexer::TokenType;

use super::parser::ParserError;


#[derive(Debug)]
pub struct Program {
    // pub exprs: Vec<Expr>,
    pub expr: Result<Expr, ParserError>,
}

// impl Ast for Program {}
impl Program {}


#[derive(Debug)]
pub enum Literal {
    Integer(i32),
    FloatingPoint(f64),
    String(String),
    Boolean(bool)
}


// impl Ast for Literal {}
impl Literal {}


#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expr>
}

// impl Ast for Atom {}
impl Grouping {}


// #[derive(Debug)]
// pub enum Operator {
//     UnaryPlus,
//     UnaryMinus,
//     LogicalNegate,

//     Plus, Minus,
//     Multiply, Subtract,
//     Modules,

//     Less, LessEqual,
//     Greater, GreaterEqual,
//     Equal, NotEqual,

//     LogicalAnd, LogicalOr,

//     Call, Index
// }

// impl Ast for Operator {}
// impl Operator {}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: TokenType,
    pub right: Box<Expr>,
}

// impl Ast for OpExpr {}
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

// impl Ast for Expr {}
impl Expr {}