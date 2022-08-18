use super::*;
use crate::lexer::lexer::Lexer;
use parser::ast::{ Program };


pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        Parser {
            lexer,
        }
    }

    pub fn parse_program(&mut self) -> Program {

        Program {
            exprs: vec![]
        }
    }
}