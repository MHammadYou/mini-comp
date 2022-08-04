
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
