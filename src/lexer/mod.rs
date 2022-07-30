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

pub struct Punctuation {
    pub raw: char,
    pub kind: PunctuationKind
}

#[derive(Debug)]
pub enum NumericHint {
    Integer,
    FloatingPoint,
    Any
}


#[derive(Debug)]
pub enum TokenType {
    EOF,
    Punctuation { raw: char, kind: PunctuationKind},
    Operations(String),
    Identifier(String),
    Char(char),
    Numeric{ raw: String, hint: NumericHint },
    String(String),
    Terminal(String)
}

#[derive(Debug)]
pub enum PunctuationKind {
    Open(BalancingDepthType),
    Close(BalancingDepthType),
    Separator,
    Equal
}

type BalancingDepthType = i32;
