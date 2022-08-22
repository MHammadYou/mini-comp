use super::*;
use crate::lexer::{ TokenType, OperationKind, PunctuationKind, NumericHint };
use parser::ast::{ Program, Expr, BinaryExpr, UnaryExpr, Literal, Grouping, Terminal };


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

            let operator: &TokenType;

            match self.previous() {
                Some(token_type) => {
                    operator = token_type
                },
                None => return Err(ParserError::InvalidOperator(String::from("Invalid Operator")))
            }
            
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
            
                let operator: &TokenType;

                match self.previous() {
                    Some(token_type) => {
                        operator = token_type
                    },
                    None => return Err(ParserError::InvalidOperator(String::from("Invalid Operator")))
                }
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
            &TokenType::Operations { raw: '-', kind: OperationKind::Minus }
            ]) {
                let operator: &TokenType;

                match self.previous() {
                    Some(token_type) => {
                        operator = token_type
                    },
                    None => return Err(ParserError::InvalidOperator(String::from("Invalid Operator")))
                }

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


        /*
            Parse Terminals
        */

        if self.match_type(&[&TokenType::Terminal(String::from("true"))]) {
            let expr = Literal::Terminal(Terminal{ value: Box::new("true")});
            return Ok(Expr::Literal(expr))
        }

        if self.match_type(&[&TokenType::Terminal(String::from("false"))]) {
            let expr = Literal::Terminal(Terminal{ value: Box::new("false")});
            return Ok(Expr::Literal(expr))
        }

        if self.match_type(&[&TokenType::Terminal(String::from("nil"))]) {
            let expr = Literal::Terminal(Terminal{ value: Box::new("nil")});
            return Ok(Expr::Literal(expr))
        }


        /*
            Parse Numerics
        */

        let hint = match self.peek() {
            Some(token_type) => match token_type {
                TokenType::Numeric { raw: _, hint } => hint,
                _ => &NumericHint::Any
            },
            None => &NumericHint::Any,
        };

        if *hint == NumericHint::Integer {

            let token = match self.peek() {
                Some(token_type) => token_type,
                None => &TokenType::EOF,
            };
            
            let value_str = match token {
                TokenType::Numeric { raw, hint: _ } => String::from(&raw[..]),
                _ => "Nil".to_string()
            };

            let value = value_str.parse::<i32>().unwrap();
            let expr = Literal::Integer(value);

            self.advance();

            return Ok(Expr::Literal(expr));
        } else if *hint == NumericHint::FloatingPoint {

            /*
                Parse Floats
            */

            let token = self.peek().unwrap();
            let value_str = match token {
                TokenType::Numeric { raw, hint: _ } => String::from(&raw[..]),
                _ => "Nil".to_string()
            };

            let value = value_str.parse::<f64>().unwrap();
            let expr = Literal::FloatingPoint(value);

            self.advance();

            return Ok(Expr::Literal(expr))
        }


        /*
            Parse Strings
        */
        let raw = self.peek().unwrap();
        let raw = match raw {
            TokenType::String(raw) => String::from(&raw[..]),
            _ => "Nil".to_string()
        };

        if self.match_type(&[&TokenType::String(String::from(&raw[..]))]) {
            let expr = Literal::String(String::from(raw));
            return Ok(Expr::Literal(expr))
        }

        /*
            Parse Grouping ()
        */

        if self.match_type(&[&TokenType::Punctuation { raw: '(', kind: PunctuationKind::Open(0) }]) {
            let expr = self.parse_expr().unwrap();
            self.consume_unit(&TokenType::Punctuation { raw: ')', kind: PunctuationKind::Close(0) });
            
            let expr = Grouping { expr: Box::new(expr) };
            return Ok(Expr::Grouping(expr))
        }

        Err(ParserError::None)
    }

    fn consume_unit(&mut self, token_type: &TokenType) {
        if self.check_type(token_type) {
            self.advance();
            return;
        }
        panic!("Invalid syntax, Expected )");
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
    None,
    InvalidOperator(String),
    InvalidExpression(String)
}