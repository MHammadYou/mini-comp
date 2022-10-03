pub mod lexer;

extern crate thiserror;

use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Some IO Error")]
    FileIO(#[from] io::Error),

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
}


#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum PunctuationKind {
    Open,
    Close,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Separator,
    Equal,
    Bang // !
}
