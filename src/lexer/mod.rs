pub mod lexer;
pub mod token;

pub use token::{TokenType, NumericHint, OperationKind, PunctuationKind, OperatorKind};

extern crate thiserror;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Some IO Error")]
    FileIO(#[from] std::io::Error),

    #[error("Expected {expected:?}, found {found:?}")]
    MissingExpectedSymbol {
        expected: &'static str,
        found: Token,
    },

    #[error("Can't find opening symbol for {symbol:?}")]
    MissingBalancedSymbol { symbol: char, open: char },

    #[error("Can't create numeric literal due to invalid character {raw:?}")]
    NumericLiteralInvalidChar { raw: String, invalid: char },

    #[error("Unrecognized symbol")]
    UnknownSymbol { symbol: String },
}