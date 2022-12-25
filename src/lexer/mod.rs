use std::fmt::Debug;

pub mod lexer;

extern crate thiserror;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Some IO Error")]
    FileIO(#[from] std::io::Error),

    #[error("Expected {expected:?}, found {found:?}")]
    MissingExpectedSymbol {
        expected: &'static str,
        found: Token
    },

    #[error("Can't find opening symbol for {symbol:?}")]
    MissingBalancedSymbol {
        symbol: char,
        open: char
    },

    #[error("Can't create numeric literal due to invalid character {raw:?}")]
    NumericLiteralInvalidChar {
        raw: String,
        invalid: char
    },

    #[error("Unrecognized symbol")]
    UnknownSymbol {
        symbol: String
    },
}

pub type Token = TokenType;

#[derive(PartialEq)]
pub struct Punctuation {
    pub raw: char,
    pub kind: PunctuationKind
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumericHint {
    Integer,
    FloatingPoint,
    Any
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperationKind {
    Plus,
    Minus,
    Star,
    Slash,
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
}

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
    Dot
}


#[derive(PartialEq, Clone)]
pub enum TokenType {
    EOF,
    Punctuation { raw: char, kind: PunctuationKind },
    Operations{ raw: char, kind: OperationKind },
    Operator(OperatorKind),
    Identifier(String),
    Char(char),
    Numeric{ raw: String, hint: NumericHint },
    String(String),
    Terminal(String)
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOF => write!(f, "EOF"),
            Self::Punctuation { raw, kind } => write!(f, "{}", raw),
            Self::Operations { raw, kind } => write!(f, "{}", raw),
            Self::Operator(arg0) => write!(f, "{:#?}", arg0),
            Self::Identifier(arg0) => write!(f, "{}", arg0),
            Self::Char(arg0) => write!(f, "{}", arg0),
            Self::Numeric { raw, hint } => write!(f, "{}", raw),
            Self::String(arg0) => write!(f, "{}", arg0),
            Self::Terminal(arg0) => write!(f, "{}", arg0),
        }
    }
}