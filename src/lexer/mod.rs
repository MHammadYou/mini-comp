mod lexer;

extern crate thiserror;

use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Some IO Error")]
    FileIO(#[from] io::Error),

    #[error("Expected {expected:?}, found {found:?}")]
    MissingExpectedSymbol {
        expected: TokenType,
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
    FloatingPoint
}


#[derive(Debug)]
pub enum TokenType {
    EOF,
    Punctuation { raw: char, kind: PunctuationKind},
    Operations(String),
    Identifier(String),
    Char(char),
    Numeric{ raw: String, hint: NumericHint },
}

#[derive(Debug)]
pub enum PunctuationKind {
    Open(BalancingDepthType),
    Close(BalancingDepthType),
    Separator,
}

type BalancingDepthType = i32;
