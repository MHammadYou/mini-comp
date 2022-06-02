
extern crate thiserror;

use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Some IO Error")]
    FileIO(#[from] io::Error),

    #[error("Expected ??, found ??")]
    MissingExpectedSymbol {
        expected: TokenType,
        found: Token
    }
}

pub type Token = TokenType;

#[derive(Debug)]
pub enum TokenType {
    EOF,
    Punctuation { raw: char, kind: PunctuationKind},
    Operations(String),
    Identifier(String),
    Char(char),
    Numeric(String),
    Unknown(char)
}

#[derive(Debug)]
pub enum PunctuationKind {
    Open(usize),
    Close(usize),
    Separator,
}

pub struct Lexer<'a> {
    pub cur_line: usize,
    pub cur_col: usize,

    pub codepoint_offset: usize,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}
