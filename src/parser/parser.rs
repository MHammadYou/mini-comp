use super::*;
use crate::lexer::{TokenType, OperationKind};
use parser::ast::{ Program, Expr, BinaryExpr, UnaryExpr, Literal, Grouping };


pub struct Parser {
    current: usize,
    tokens: Vec<TokenType>
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Parser {
        Parser {
            current: 0,
            tokens
        }
    }

    pub fn parse_program(&mut self) -> Program {

        Program {
            expr: self.parse_expr()
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Expr, ParserError> {


        let mut expr = self.parse_factor().unwrap();

        while self.match_type(&[&TokenType::Operations { raw: '+', kind: OperationKind::Plus }]) {

            let operator = self.previous().unwrap();
            
            // Tring to get the actual obj instead of refernce
            // can't think of anything else right now
            let operator = match operator {

                // todo: I'll clean this part later

                TokenType::Operations { raw, kind: OperationKind::Plus } => TokenType::Operations { raw: *raw, kind: OperationKind::Plus },
                TokenType::Operations { raw, kind: OperationKind::Minus } => TokenType::Operations { raw: *raw, kind: OperationKind::Minus },
                _ => TokenType::EOF
            };

            let right = self.parse_factor().unwrap();

            let new_expr = BinaryExpr{
                left: Box::new(expr),
                op: operator,
                right: Box::new(right)
            };

            expr = Expr::BinaryExpr(new_expr)

        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParserError> {
        let expr = BinaryExpr{
            left: Box::new(
                Expr::Literal(Literal::Integer(34))
            ),
            op: TokenType::Operations { raw: '+', kind: OperationKind::Plus },
            right: Box::new(
                Expr::Literal(Literal::Integer(32))
            )
        };
        
        Ok(Expr::BinaryExpr(expr))
    }

    fn match_type(&mut self, types: &[&TokenType]) -> bool {
        for token_type in types {
            if self.check_type(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check_type(&mut self, token_type: &TokenType) -> bool {
        if self.end_of_stream() {
            return false
        } 
        match self.peek() {
            Some(token) => {
                token == token_type 
            },
            None => false
        }
    }

    fn advance(&mut self) -> Option<&TokenType> {
        if !self.end_of_stream() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&mut self) -> Option<&TokenType> {
        self.tokens.get(self.current)
    }

    fn previous(&mut self) -> Option<&TokenType> {
        self.tokens.get(self.current - 1)
    }

    fn end_of_stream(&mut self) -> bool {
        match self.peek() {
            Some(token_type) => {
                *token_type == TokenType::EOF
            },
            None => false

        }
    }


}


#[derive(Debug)]
pub enum ParserError {
    None
}