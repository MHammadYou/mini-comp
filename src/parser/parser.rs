use super::*;
use crate::lexer::TokenType;
use parser::ast::{ Program, Expr, BinaryExpr, UnaryExpr, Literal, Grouping, Operator };


pub struct Parser {
    current: i32,
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
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, ParserError> {


        let expr = self.parse_comparison();




        expr
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParserError> {
        let expr = BinaryExpr{
            left: Box::new(
                Expr::Literal(Literal::Integer(34))
            ),
            op: Operator::Plus,
            right: Box::new(
                Expr::Literal(Literal::Integer(32))
            )
        };
        
        Ok(Expr::BinaryExpr(expr))
    }


}


pub enum ParserError {
    None
}