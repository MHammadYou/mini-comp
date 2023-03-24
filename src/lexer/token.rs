#[derive(Debug, PartialEq, Clone)]
pub enum PunctuationKind {
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Separator,
    Equal,
    Bang,
    Comma,
    Dot,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorKind {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualEqual,
    BangEqual,
    PlusEqual,
    MinusEqual,
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperationKind {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumericHint {
    Integer,
    FloatingPoint,
    Any,
}

#[derive(PartialEq)]
pub struct Punctuation {
    pub raw: char,
    pub kind: PunctuationKind,
}


#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    EOF,
    Punctuation { raw: char, kind: PunctuationKind },
    Operations { raw: char, kind: OperationKind },
    Operator(OperatorKind),
    Identifier(String),
    Char(char),
    Numeric { raw: String, hint: NumericHint },
    String(String),
    Terminal(String),
}
