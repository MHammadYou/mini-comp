
pub struct Program {

}

pub enum Literal {
    Integer(i32),
    FloatingPoint(f64),
    String(String),
    Boolean(bool)
}

pub enum Atom {
    Identifier(String),
    Literal(Literal),

}
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
}

pub struct OpExpr {
    pub op: Operator,
}