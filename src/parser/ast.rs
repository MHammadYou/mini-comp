// pub trait Ast : std::fmt::Debug {
//
// }
//
// #[derive(Debug)]


use super::parser::ParserError;


pub struct Program {
    // pub exprs: Vec<Expr>,
    pub expr: Result<Expr, ParserError>,
}

// impl Ast for Program {}
impl Program {}


// #[derive(Debug)]
pub enum Literal {
    Integer(i32),
    FloatingPoint(f64),
    String(String),
    Boolean(bool)
}


// impl Ast for Literal {}
impl Literal {}


// #[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expr>
}

// impl Ast for Atom {}
impl Grouping {}


// #[derive(Debug)]
pub enum Operator {
    UnaryPlus,
    UnaryMinus,
    LogicalNegate,

    Plus, Minus,
    Multiply, Subtract,
    Modules,

    Less, LessEqual,
    Greater, GreaterEqual,
    Equal, NotEqual,

    LogicalAnd, LogicalOr,

    Call, Index
}

// impl Ast for Operator {}
impl Operator {}

// #[derive(Debug)]
pub struct UnaryExpr {
    pub op: Operator,
    pub args: Vec<Expr>,
}

// impl Ast for OpExpr {}
impl UnaryExpr {}


pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>
}

impl BinaryExpr {}



// #[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    Grouping(Grouping),
    Literal(Literal)
}

// impl Ast for Expr {}
impl Expr {}