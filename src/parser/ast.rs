pub trait Ast : std::fmt::Debug {

}

#[derive(Debug)]
pub struct Program {
    pub exprs: Vec<Expr>,
}

impl Ast for Program {}

#[derive(Debug)]
pub enum Literal {
    Integer(i32),
    FloatingPoint(f64),
    String(String),
    Boolean(bool)
}


impl Ast for Literal {}

#[derive(Debug)]
pub enum Atom {
    Identifier(String),
    Literal(Literal),
}

impl Ast for Atom {}


#[derive(Debug)]
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

impl Ast for Operator {}


#[derive(Debug)]
pub struct OpExpr {
    pub op: Operator,
    pub args: Vec<Expr>,
}

impl Ast for OpExpr {}


#[derive(Debug)]
pub enum Expr {
    OpExpr(Operator),
}

impl Ast for Expr {}
