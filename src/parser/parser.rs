use super::{*, stmt::Stmt};
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

    pub fn parse_program_(&mut self) -> Vec<Stmt> {
        let statements = vec![];

        while !self.end_of_stream() {
            statements.push(self.parse_statement());
        }

        return statements;
    }

    fn parse_statement(&mut self) -> Stmt {
        if self.match_type(&[&TokenType::Terminal(String::from("print"))]) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.parse_expr();
        self.consume_unit(&TokenType::Punctuation { raw: ';', kind: PunctuationKind::Separator });

        return Stmt::Print(expr);

    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.parse_expr();
        self.consume_unit(&TokenType::Punctuation { raw: ';', kind: PunctuationKind::Separator });

        return Stmt::Expression(expr);
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {

        let mut expr = self.parse_factor();

        while self.match_type(&[
            &TokenType::Operations { raw: '+', kind: OperationKind::Plus }, 
            &TokenType::Operations { raw: '-', kind: OperationKind::Minus }
            ]) {

            let operator: &TokenType;

            match self.previous() {
                Some(token_type) => {
                    operator = token_type
                },
                None => ()
            }

            let operator = operator.clone();

            let right = self.parse_factor();

            let new_expr = BinaryExpr{
                left: Box::new(expr),
                op: operator,
                right: Box::new(right)
            };

            expr = Expr::BinaryExpr(new_expr)

        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {

        let mut expr = self.parse_unary();

        while self.match_type(&[
            &TokenType::Operations { raw: '*', kind: OperationKind::Star }, 
            &TokenType::Operations { raw: '/', kind: OperationKind::Slash }
            ]) {
            
                let operator: &TokenType;

                match self.previous() {
                    Some(token_type) => {
                        operator = token_type
                    },
                    None => ()
                }
                let operator = operator.clone();

                let right = self.parse_unary();

                let new_expr = BinaryExpr{
                    left: Box::new(expr),
                    op: operator,
                    right: Box::new(right)
                };

                expr = Expr::BinaryExpr(new_expr)
            }

        expr
    }


    fn parse_unary(&mut self) -> Expr {
        
        if self.match_type(&[
            &TokenType::Punctuation { raw: '!', kind: PunctuationKind::Bang }, 
            &TokenType::Operations { raw: '-', kind: OperationKind::Minus }
            ]) {
                let operator: &TokenType;

                match self.previous() {
                    Some(token_type) => {
                        operator = token_type
                    },
                    None => ()
                }

                let operator = operator.clone();

                let right = self.parse_unary();

                let new_expr = UnaryExpr {
                    op: operator,
                    right: Box::new(right)
                };

                return Expr::UnaryExpr(new_expr)
            }
        
        self.parse_literal()
    }

    fn parse_literal(&mut self) -> Expr {


        /*
            Parse Terminals
        */

        if self.match_type(&[&TokenType::Terminal(String::from("true"))]) {
            let expr = Literal::Terminal(Terminal{ value: Box::new("true")});
            return Expr::Literal(expr)
        }

        if self.match_type(&[&TokenType::Terminal(String::from("false"))]) {
            let expr = Literal::Terminal(Terminal{ value: Box::new("false")});
            return Expr::Literal(expr)
        }

        if self.match_type(&[&TokenType::Terminal(String::from("nil"))]) {
            let expr = Literal::Terminal(Terminal{ value: Box::new("nil")});
            return Expr::Literal(expr)
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

            let token: &TokenType;
            match self.peek() {
                Some(token_type) => {
                    token = token_type
                },
                None => ()
            }
            
            let value_str = match token {
                TokenType::Numeric { raw, hint: _ } => String::from(&raw[..]),
                _ => "Nil".to_string()
            };

            let value = value_str.parse::<i32>().unwrap();
            let expr = Literal::Integer(value);

            self.advance();

            return Expr::Literal(expr);
        } else if *hint == NumericHint::FloatingPoint {

            /*
                Parse Floats
            */

            let token: &TokenType;
            match self.peek() {
                Some(token_type) => {
                    token = token_type
                },
                None => ()
            }

            let value_str = match token {
                TokenType::Numeric { raw, hint: _ } => String::from(&raw[..]),
                _ => "Nil".to_string()
            };

            let value: f64 = value_str.parse::<f64>().unwrap();
            let expr = Literal::FloatingPoint(value);

            self.advance();

            return Expr::Literal(expr)
        }


        /*
            Parse Strings
        */

        let raw = match self.peek() {
            Some(token_type) => token_type,
            None => &TokenType::EOF
        };

        let raw = match raw {
            TokenType::String(raw) => String::from(&raw[..]),
            _ => "Nil".to_string()
        };

        if self.match_type(&[&TokenType::String(String::from(&raw[..]))]) {
            let expr = Literal::String(String::from(raw));
            return Expr::Literal(expr)
        }

        /*
            Parse Grouping ()
        */

        if self.match_type(&[&TokenType::Punctuation { raw: '(', kind: PunctuationKind::Open(0) }]) {

            let expr = self.parse_expr();

            self.consume_unit(&TokenType::Punctuation { raw: ')', kind: PunctuationKind::Close(0) });
            
            let expr = Grouping { expr: Box::new(expr) };
            return Expr::Grouping(expr)
        }

        panic!("Invalid Syntax, No literal match");
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

    fn peek(&self) -> Option<&TokenType> {
        self.tokens.get(self.current)
    }

    fn previous(&mut self) -> Option<&TokenType> {
        self.tokens.get(self.current - 1)
    }

    fn end_of_stream(&self) -> bool {
        match self.peek() {
            Some(token_type) => {
                *token_type == TokenType::EOF
            },
            None => false
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.end_of_stream() {
            if let Some(token_type) = self.previous() {
                match token_type {
                    TokenType::Punctuation { raw: ';', kind: PunctuationKind::Separator } => return,
                    _ => ()
                }
            }

            if let Some(peek) = self.peek() {
                match peek {
                    TokenType::Terminal(str) => {
                        if str == "let" || str == "def" || str == "print" {
                            return;
                        }
                    },
                    _ => ()
                }
            }
            self.advance();
        }
    }
}


#[derive(Debug)]
pub enum ParserError {
    None,
    InvalidOperator(String),
    InvalidExpression(String),
    InvalidLiteral(String)
}