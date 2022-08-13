use super::*;
use crate::lexer::lexer::Lexer;
use parser::ast::{Program, Ast};


pub type ErrorConsumer = fn(TaggedNode<Box<dyn Ast>>) -> ();


pub struct Parser<'a> {
    lexer: Lexer<'a>,
    error_consumer: ErrorConsumer
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, error_consumer: ErrorConsumer) -> Parser {
        Parser {
            lexer,
            error_consumer
        }
    }

    pub fn parse_program(&mut self) -> Program {

        // TODO: fix here
        self.error_consumer.call((TaggedNode{ ast: Box::new(Program { exprs: vec![] }), err: ParserError::None }));


        Program {
            exprs: vec![]
        }
    }
}

pub struct TaggedNode<Ast> {
    pub ast: Ast,
    pub err: ParserError
}

pub enum ParserError {
    None
}