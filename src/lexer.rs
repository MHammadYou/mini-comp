
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
    },
    
    #[error("Can't find opening symbol for {symbol:?}")]
    MissingBalancedSymbol {
        symbol: String,
    },

    #[error("Unrecognized symbol")]
    UnknownSymbol {
      symbol: String
    },
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
}

#[derive(Debug)]
pub enum PunctuationKind {
    Open(BalancingDepthType),
    Close(BalancingDepthType),
    Separator,
}

type BalancingDepthType = i32;

pub struct Lexer<'a> {
    pub cur_line: usize,
    pub cur_col: usize,

    pub codepoint_offset: usize,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    balancing_state: std::collections::HashMap<char, BalancingDepthType>,
}


impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer {
            cur_line: 1,
            cur_col: 1,
            codepoint_offset: 0,
            chars: chars.chars().peekable(),
            balancing_state: std::collections::HashMap::new(),
        }
    }

    fn push_symbol(&mut self, c: &char) -> BalancingDepthType {
        if let Some(v) = self.balancing_state.get_mut(&c) {
            *v += 1;
            *v
        } else {
            self.balancing_state.insert(*c, 0);
            0
        }
    }
    
    fn pop_symbol(&mut self, c: &char) -> Result<BalancingDepthType, LexerError> {
        if let Some(v) = self.balancing_state.get_mut(&c) {
            if *v == 1 {
                *v -= 1;
                Ok(*v)
            } else {
                Err(LexerError::MissingBalancedSymbol { symbol: String::from(*c) })
            }
        } else {
            Err(LexerError::MissingBalancedSymbol { symbol: String::from(*c) })
        }
    }

    fn transform_to_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Open(self.push_symbol(&c)) }),
            ')' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Close(self.pop_symbol(&c)?) }),
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string() })
        }
    }
}