use super::*;
use crate::lexer::{ TokenType, OperationKind, PunctuationKind, NumericHint };
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

        while self.match_type(&[
            &TokenType::Operations { raw: '+', kind: OperationKind::Plus }, 
            &TokenType::Operations { raw: '-', kind: OperationKind::Minus }
            ]) {

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
        let mut expr = self.parse_unary().unwrap();

        while self.match_type(&[
            &TokenType::Operations { raw: '*', kind: OperationKind::Star }, 
            &TokenType::Operations { raw: '/', kind: OperationKind::Slash }
            ]) {
            
                let operator = self.previous().unwrap();
                let operator = match operator {

                    TokenType::Operations { raw, kind: OperationKind::Star } => TokenType::Operations { raw: *raw, kind: OperationKind::Star },
                    TokenType::Operations { raw, kind: OperationKind::Slash } => TokenType::Operations { raw: *raw, kind: OperationKind::Slash },
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


    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
        
        if self.match_type(&[
            &TokenType::Punctuation { raw: '!', kind: PunctuationKind::Bang }, 
            &TokenType::Operations { raw: '-', kind: OperationKind::Plus }
            ]) {
                let operator = self.previous().unwrap();
                let operator = match operator {
                    TokenType::Punctuation { raw, kind: PunctuationKind::Bang } => TokenType::Punctuation { raw: *raw, kind: PunctuationKind::Bang },
                    TokenType::Operations { raw, kind: OperationKind::Minus } => TokenType::Operations { raw: *raw, kind: OperationKind::Minus },
                    _ => TokenType::EOF
                };

                let right  = self.parse_unary().unwrap();
                let new_expr = UnaryExpr {
                    op: operator,
                    right: Box::new(right)
                };

                return Ok(Expr::UnaryExpr(new_expr))
            }
        
        self.parse_literal()
    }

    fn parse_literal(&mut self) -> Result<Expr, ParserError> {

        // let hint = match self.peek() {
        //     Some(token_type) => match token_type {
        //         TokenType::Numeric { raw, hint } => hint,
        //         _ => &NumericHint::Any
        //     },
        //     None => &NumericHint::Any,
        // };

        // match hint {
        //     NumericHint::Integer => {
        //         let token_type = match self.previous().unwrap() {
        //             TokenType::Numeric { raw, hint: NumericHint::Integer } => TokenType::Numeric { raw: raw.to_string(), hint: NumericHint::Integer },
        //             _ => TokenType::EOF
        //         };

        //         let value_str = match token_type {
        //             TokenType::Numeric { raw, hint: _ } => raw,
        //             _ => "Nil".to_string(),
        //         };
        //         let value = value_str.parse::<i32>().unwrap();
        //         let expr = Literal::Integer(value);
        //         return Ok(Expr::Literal(expr))
        //     },
        //     _ => return Err(ParserError::None)
        // }


        if self.match_type(&[
            &TokenType::Numeric { raw: "10".to_string(), hint: NumericHint::Integer } 
            ]) {

                let token_type = match self.previous().unwrap() {
                    TokenType::Numeric { raw, hint: NumericHint::Integer } => TokenType::Numeric { raw: raw.to_string(), hint: NumericHint::Integer },
                    _ => TokenType::EOF
                };

                let value_str = match token_type {
                    TokenType::Numeric { raw, hint: _ } => raw,
                    _ => "Nil".to_string(),
                };
                let value = value_str.parse::<i32>().unwrap();
                let expr = Literal::Integer(value);
                return Ok(Expr::Literal(expr))
            }

            else {
                Err(ParserError::None)
            }
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