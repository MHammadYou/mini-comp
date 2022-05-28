
use std::io;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("")]
    FileIO(#[from] io::Error),

    MissingExpectedSymbol {
        expected: ??,
        found: ??
    }
}

pub type Token = TokenType;

pub enum TokenType {
    EOF,
    Punctuation { raw: char, kind: PunctuationKind},
    Operations(String),
    Identifier(String),
    Char(char),
    Numeric(String)
}

pub enum PunctuationKind {
    Open(usize),
    Close(usize),
    Separator,
}