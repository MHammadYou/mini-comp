extern crate thiserror;
use thiserror::Error;

use super::TokenType;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Some IO Error")]
    FileIO(#[from] std::io::Error),

    #[error("Expected {expected:?}, found {found:?}")]
    MissingExpectedSymbol {
        expected: &'static str,
        found: TokenType,
    },

    #[error("Can't find opening symbol for {symbol:?}")]
    MissingBalancedSymbol { symbol: char, open: char },

    #[error("Can't create numeric literal due to invalid character {raw:?}")]
    NumericLiteralInvalidChar { raw: String, invalid: char },

    #[error("Unrecognized symbol")]
    UnknownSymbol { symbol: String },
}