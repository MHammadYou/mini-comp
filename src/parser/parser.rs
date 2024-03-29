use super::*;
use crate::lexer::{NumericHint, OperationKind, OperatorKind, PunctuationKind, TokenType};
use parser::expr::*;
use stmt::Stmt;

pub struct Parser {
    current: usize,
    tokens: Vec<TokenType>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Parser {
        Parser { current: 0, tokens }
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.end_of_stream() {
            statements.push(self.parse_declaration());
        }

        return statements;
    }

    fn parse_declaration(&mut self) -> Stmt {
        if self.match_type(&[&TokenType::Terminal(String::from("let"))]) {
            return self.let_declaration();
        }

        if self.match_type(&[&TokenType::Terminal(String::from("def"))]) {
            return self.function_statement("function");
        }

        if self.match_type(&[&TokenType::Terminal(String::from("class"))]) {
            return self.class_statement();
        }

        return self.parse_statement();
    }

    fn class_statement(&mut self) -> Stmt {
        let ident: String = match self.peek() {
            TokenType::Identifier(value) => value,
            _ => String::from("Invalid"),
        };

        let name = self.consume_unit(&TokenType::Identifier(ident), "Expected class name.");

        let mut super_class = None;

        if self.match_type(&[&TokenType::Terminal(String::from("extends"))]) {
            let ident: String = match self.peek() {
                TokenType::Identifier(value) => value,
                _ => String::from("Invalid"),
            };

            self.consume_unit(&TokenType::Identifier(ident), "Expected parent classname.");
            super_class = Some(Expr::Variable(self.previous()));
        }

        self.consume_unit(
            &TokenType::Punctuation {
                raw: '{',
                kind: PunctuationKind::OpenCurly,
            },
            "Expected '{' after class name.",
        );

        let mut methods = vec![];

        while !self.check_type(&TokenType::Punctuation {
            raw: '}',
            kind: PunctuationKind::CloseCurly,
        }) && !self.end_of_stream()
        {
            methods.push(self.function_statement("method"));
        }

        self.consume_unit(
            &TokenType::Punctuation {
                raw: '}',
                kind: PunctuationKind::CloseCurly,
            },
            "Expected '}' after class body",
        );

        Stmt::Class {
            name,
            super_class,
            methods,
        }
    }

    fn let_declaration(&mut self) -> Stmt {
        let ident: String = match self.peek() {
            TokenType::Identifier(value) => value,
            _ => String::from("Invalid"),
        };
        let name = self.consume_unit(
            &TokenType::Identifier(ident),
            "Expected variable name after let",
        );

        if self.match_type(&[&TokenType::Punctuation {
            raw: '=',
            kind: PunctuationKind::Equal,
        }]) {
            let initilizer = self.parse_expr();
            self.consume_unit(
                &TokenType::Punctuation {
                    raw: ';',
                    kind: PunctuationKind::Separator,
                },
                "Expected ; after expression",
            );
            return Stmt::Let {
                token: name,
                initilizer,
            };
        }
        panic!("Invalid syntax, Expected '=' after variable name")
    }

    fn parse_statement(&mut self) -> Stmt {
        if self.match_type(&[&TokenType::Terminal(String::from("print"))]) {
            return self.print_statement();
        }

        if self.match_type(&[&TokenType::Terminal(String::from("if"))]) {
            return self.if_statement();
        }

        if self.match_type(&[&TokenType::Terminal(String::from("while"))]) {
            return self.while_statement();
        }

        if self.match_type(&[&TokenType::Terminal(String::from("for"))]) {
            return self.for_statement();
        }

        if self.match_type(&[&TokenType::Terminal(String::from("return"))]) {
            return self.return_statement();
        }

        if self.match_type(&[&TokenType::Punctuation {
            raw: '{',
            kind: PunctuationKind::OpenCurly,
        }]) {
            return Stmt::Block {
                statements: self.parse_block(),
            };
        }

        return self.expression_statement();
    }

    fn return_statement(&mut self) -> Stmt {
        let keyword = self.previous();

        let mut value = None;

        if !self.check_type(&TokenType::Punctuation {
            raw: ';',
            kind: PunctuationKind::Separator,
        }) {
            let expr = self.parse_expr();

            value = Some(expr);
        }

        self.consume_unit(
            &TokenType::Punctuation {
                raw: ';',
                kind: PunctuationKind::Separator,
            },
            "Expected ';' after return value",
        );

        Stmt::Return { keyword, value }
    }

    fn function_statement(&mut self, kind: &str) -> Stmt {
        let ident = match self.peek() {
            TokenType::Identifier(value) => value,
            _ => String::from("Invalid"),
        };

        let name = self.consume_unit(
            &TokenType::Identifier(ident),
            "Expected function name after def",
        );

        self.consume_unit(
            &TokenType::Punctuation {
                raw: '(',
                kind: PunctuationKind::OpenParen,
            },
            "Expected '(' after function name",
        );

        let mut parameters = vec![];

        if !self.check_type(&TokenType::Punctuation {
            raw: ')',
            kind: PunctuationKind::CloseParen,
        }) {
            let ident: String = match self.peek() {
                TokenType::Identifier(value) => value,
                _ => String::from("Invalid"),
            };

            parameters.push(self.consume_unit(
                &TokenType::Identifier(ident),
                "Expected parameter after '('",
            ));

            while self.match_type(&[&TokenType::Punctuation {
                raw: ',',
                kind: PunctuationKind::Comma,
            }]) {
                if parameters.len() >= 255 {
                    panic!("Can't have more than 255 parameters.");
                }

                let ident: String = match self.peek() {
                    TokenType::Identifier(value) => value,
                    _ => String::from("Invalid"),
                };

                parameters.push(self.consume_unit(
                    &TokenType::Identifier(ident),
                    "Expected parameter after ','",
                ));
            }
        }

        self.consume_unit(
            &TokenType::Punctuation {
                raw: ')',
                kind: PunctuationKind::CloseParen,
            },
            "Expected ')' after for",
        );

        self.consume_unit(
            &TokenType::Punctuation {
                raw: '{',
                kind: PunctuationKind::OpenCurly,
            },
            &format!("Expected '{{' before {} body.", kind),
        );

        let body = self.parse_block();

        Stmt::Function {
            name,
            params: parameters,
            body,
        }
    }

    fn for_statement(&mut self) -> Stmt {
        self.consume_unit(
            &TokenType::Punctuation {
                raw: '(',
                kind: PunctuationKind::OpenParen,
            },
            "Expected '(' after for",
        );

        let initilizer;
        if self.match_type(&[&TokenType::Terminal(String::from("let"))]) {
            initilizer = self.let_declaration();
        } else {
            initilizer = self.expression_statement();
        }

        let _condition = self.parse_expr();
        self.consume_unit(
            &TokenType::Punctuation {
                raw: ';',
                kind: PunctuationKind::Separator,
            },
            "Expected ';' after for loop condition",
        );

        let change = self.parse_expr();

        self.consume_unit(
            &TokenType::Punctuation {
                raw: ')',
                kind: PunctuationKind::CloseParen,
            },
            "Expected ')' after for",
        );

        let mut body = self.parse_statement();

        body = Stmt::Block {
            statements: vec![body, Stmt::Expression(change)],
        };

        body = Stmt::While {
            condition: Expr::Literal(Literal::Boolean(true)),
            body: Box::new(body),
        };

        body = Stmt::Block {
            statements: vec![initilizer, body],
        };

        body
    }

    fn while_statement(&mut self) -> Stmt {
        self.consume_unit(
            &TokenType::Punctuation {
                raw: '(',
                kind: PunctuationKind::OpenParen,
            },
            "Expected '(' after while",
        );
        let condition = self.parse_expr();
        self.consume_unit(
            &TokenType::Punctuation {
                raw: ')',
                kind: PunctuationKind::CloseParen,
            },
            "Expected ')' after expression",
        );

        let statment = self.parse_statement();
        Stmt::While {
            condition,
            body: Box::new(statment),
        }
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume_unit(
            &TokenType::Punctuation {
                raw: '(',
                kind: PunctuationKind::OpenParen,
            },
            "Expected '(' after if",
        );
        let condition = self.parse_expr();
        self.consume_unit(
            &TokenType::Punctuation {
                raw: ')',
                kind: PunctuationKind::CloseParen,
            },
            "Expected ')' after expression",
        );

        let branch = self.parse_statement();

        Stmt::If {
            condition,
            branch: Box::new(branch),
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.check_type(&TokenType::Punctuation {
            raw: '}',
            kind: PunctuationKind::CloseCurly,
        }) && !self.end_of_stream()
        {
            statements.push(self.parse_declaration());
        }

        self.consume_unit(
            &TokenType::Punctuation {
                raw: '}',
                kind: PunctuationKind::CloseCurly,
            },
            "Expected '}' after block",
        );

        statements
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.parse_expr();
        self.consume_unit(
            &TokenType::Punctuation {
                raw: ';',
                kind: PunctuationKind::Separator,
            },
            "Expected ; after expresion",
        );

        return Stmt::Print(expr);
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.parse_expr();
        self.consume_unit(
            &TokenType::Punctuation {
                raw: ';',
                kind: PunctuationKind::Separator,
            },
            "Expected ; after expression",
        );

        return Stmt::Expression(expr);
    }

    fn parse_expr(&mut self) -> Expr {
        if self.match_look_ahead(&[
            &TokenType::Operator(OperatorKind::Increment),
            &TokenType::Operator(OperatorKind::Decrement),
        ]) {
            let ident = match self.peek() {
                TokenType::Identifier(value) => value,
                _ => String::from("Invalid"),
            };

            let identifier =
                self.consume_unit(&TokenType::Identifier(ident), "Expected identifier.");
            let operator = self.advance();

            match operator {
                TokenType::Operator(OperatorKind::Increment) => {
                    let new_expr = UpdateExpr {
                        name: identifier,
                        op: TokenType::Operations {
                            raw: '+',
                            kind: OperationKind::Plus,
                        },
                        change: Box::new(Expr::Literal(Literal::Integer(1))),
                    };
                    return Expr::Update(new_expr);
                }
                TokenType::Operator(OperatorKind::Decrement) => {
                    let new_expr = UpdateExpr {
                        name: identifier,
                        op: TokenType::Operations {
                            raw: '-',
                            kind: OperationKind::Minus,
                        },
                        change: Box::new(Expr::Literal(Literal::Integer(1))),
                    };
                    return Expr::Update(new_expr);
                }
                _ => {
                    panic!("Invalid operator")
                }
            }
        }
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Expr {
        let expr = self.parse_equality();

        if self.match_type(&[&TokenType::Punctuation {
            raw: '=',
            kind: PunctuationKind::Equal,
        }]) {
            let value = self.parse_assignment();

            match expr {
                Expr::Variable(ident_name) => {
                    let name = ident_name;
                    let new_expr = AssignExpr {
                        name,
                        value: Box::new(value),
                    };
                    return Expr::Assign(new_expr);
                }
                Expr::Get(get) => {
                    let new_expr = SetExpr {
                        object: get.object,
                        name: get.name,
                        value: Box::new(value),
                    };
                    return Expr::Set(new_expr);
                }
                _ => (),
            }
        } else if self.match_type(&[&TokenType::Operator(OperatorKind::PlusEqual)]) {
            let value = self.parse_assignment();

            match expr {
                Expr::Variable(ident_name) => {
                    let name = ident_name;
                    let new_expr = UpdateExpr {
                        name,
                        op: TokenType::Operations {
                            raw: '+',
                            kind: OperationKind::Plus,
                        },
                        change: Box::new(value),
                    };
                    return Expr::Update(new_expr);
                }
                _ => (),
            }
        } else if self.match_type(&[&TokenType::Operator(OperatorKind::MinusEqual)]) {
            let value = self.parse_assignment();

            match expr {
                Expr::Variable(ident_name) => {
                    let name = ident_name;
                    let new_expr = UpdateExpr {
                        name,
                        op: TokenType::Operations {
                            raw: '-',
                            kind: OperationKind::Minus,
                        },
                        change: Box::new(value),
                    };
                    return Expr::Update(new_expr);
                }
                _ => (),
            }
        }
        expr
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while self.match_type(&[
            &TokenType::Operator(OperatorKind::BangEqual),
            &TokenType::Operator(OperatorKind::EqualEqual),
        ]) {
            let operator = self.previous();

            let right = self.parse_comparison();

            let new_expr = BinaryExpr {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };

            expr = Expr::BinaryExpr(new_expr)
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while self.match_type(&[
            &TokenType::Operator(OperatorKind::Greater),
            &TokenType::Operator(OperatorKind::GreaterEqual),
            &TokenType::Operator(OperatorKind::Less),
            &TokenType::Operator(OperatorKind::LessEqual),
        ]) {
            let operator = self.previous();

            let right = self.parse_term();

            let new_expr = BinaryExpr {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };

            expr = Expr::BinaryExpr(new_expr)
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while self.match_type(&[
            &TokenType::Operations {
                raw: '+',
                kind: OperationKind::Plus,
            },
            &TokenType::Operations {
                raw: '-',
                kind: OperationKind::Minus,
            },
        ]) {
            let operator = self.previous();

            let right = self.parse_factor();

            let new_expr = BinaryExpr {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };

            expr = Expr::BinaryExpr(new_expr)
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while self.match_type(&[
            &TokenType::Operations {
                raw: '*',
                kind: OperationKind::Star,
            },
            &TokenType::Operations {
                raw: '/',
                kind: OperationKind::Slash,
            },
        ]) {
            let operator = self.previous();

            let right = self.parse_unary();

            let new_expr = BinaryExpr {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };

            expr = Expr::BinaryExpr(new_expr)
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        if self.match_type(&[
            &TokenType::Punctuation {
                raw: '!',
                kind: PunctuationKind::Bang,
            },
            &TokenType::Operations {
                raw: '-',
                kind: OperationKind::Minus,
            },
        ]) {
            let operator = self.previous();

            let right = self.parse_unary();

            let new_expr = UnaryExpr {
                op: operator,
                right: Box::new(right),
            };

            return Expr::UnaryExpr(new_expr);
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Expr {
        let mut expr = self.parse_literal();

        loop {
            if self.match_type(&[&TokenType::Punctuation {
                raw: '(',
                kind: PunctuationKind::OpenParen,
            }]) {
                expr = self.finish_call(expr);
            } else if self.match_type(&[&TokenType::Punctuation {
                raw: '.',
                kind: PunctuationKind::Dot,
            }]) {
                let ident: String = match self.peek() {
                    TokenType::Identifier(value) => value,
                    _ => String::from("Invalid"),
                };

                let name = self.consume_unit(
                    &TokenType::Identifier(ident),
                    "Expected property name after '.'",
                );
                expr = Expr::Get(GetExpr {
                    object: Box::new(expr),
                    name,
                });
            } else {
                break;
            }
        }

        expr
    }

    fn parse_literal(&mut self) -> Expr {
        /*
            Parse Terminals
        */

        if self.match_type(&[&TokenType::Terminal(String::from("true"))]) {
            let expr = Literal::Terminal(Terminal {
                value: Box::new("true"),
            });
            return Expr::Literal(expr);
        }

        if self.match_type(&[&TokenType::Terminal(String::from("false"))]) {
            let expr = Literal::Terminal(Terminal {
                value: Box::new("false"),
            });
            return Expr::Literal(expr);
        }

        if self.match_type(&[&TokenType::Terminal(String::from("nil"))]) {
            let expr = Literal::Terminal(Terminal {
                value: Box::new("nil"),
            });
            return Expr::Literal(expr);
        }

        if self.match_type(&[&TokenType::Terminal(String::from("this"))]) {
            let expr = ThisExpr {
                keyword: self.previous(),
            };
            return Expr::This(expr);
        }

        if self.match_type(&[&TokenType::Terminal(String::from("super"))]) {
            let keyword = self.previous();
            self.consume_unit(
                &TokenType::Punctuation {
                    raw: '.',
                    kind: PunctuationKind::Dot,
                },
                "Expected '.' after super keyword",
            );

            let ident = match self.peek() {
                TokenType::Identifier(value) => value,
                _ => String::from("Invalid"),
            };

            let method =
                self.consume_unit(&TokenType::Identifier(ident), "Expected superclass name.");

            let new_expr = SuperExpr { keyword, method };
            return Expr::Super(new_expr);
        }

        let value = match self.peek() {
            TokenType::Identifier(value) => value,
            _ => String::from("Invalid"),
        };

        if self.match_type(&[&TokenType::Identifier(value)]) {
            return Expr::Variable(self.previous());
        }

        /*
            Parse Numerics
        */

        match self.peek() {
            TokenType::Numeric { raw, hint } => {
                self.advance();

                match hint {
                    NumericHint::Integer => {
                        let value = raw.parse::<i32>().unwrap();
                        let expr = Literal::Integer(value);
                        return Expr::Literal(expr);
                    },
                    NumericHint::FloatingPoint => {
                        let value = raw.parse::<f64>().unwrap();
                        let expr = Literal::FloatingPoint(value);
                        return Expr::Literal(expr);
                    },
                }
            }
            _ => ()
        }

        // let hint = match self.peek() {
        //     TokenType::Numeric { raw: _, hint } => hint,
        //     _ => unreachable!(),
        // };

        // if hint == NumericHint::Integer {
            
        //     let value_str = match self.peek() {
        //         TokenType::Numeric { raw, hint: _ } => raw,
        //         _ => panic!("Invalid raw value for integer type."),
        //     };

        //     let value = value_str.parse::<i32>().unwrap();
        //     let expr = Literal::Integer(value);

        //     self.advance();
        //     return Expr::Literal(expr);
        // } else if hint == NumericHint::FloatingPoint {

        //     let value_str = match self.peek() {
        //         TokenType::Numeric { raw, hint: _ } => raw,
        //         _ => panic!("Invalid raw value for integer type."),
        //     };

        //     let value: f64 = value_str.parse::<f64>().unwrap();
        //     let expr = Literal::FloatingPoint(value);

        //     self.advance();
        //     return Expr::Literal(expr);
        // }

        /*
            Parse Strings
        */
        
        if self.peek().is_string() {
            let value = self.advance().unwrap_string();
            let expr = Literal::String(String::from(value));
            return Expr::Literal(expr);
        }

        /*
            Parse Grouping ()
        */

        if self.match_type(&[&TokenType::Punctuation {
            raw: '(',
            kind: PunctuationKind::OpenParen,
        }]) {
            let expr = self.parse_expr();

            self.consume_unit(
                &TokenType::Punctuation {
                    raw: ')',
                    kind: PunctuationKind::CloseParen,
                },
                "Expected )",
            );

            let expr = Grouping {
                expr: Box::new(expr),
            };
            return Expr::Grouping(expr);
        }

        panic!("Invalid Syntax, No literal match");
    }

    fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut args = vec![];

        if !self.check_type(&TokenType::Punctuation {
            raw: ')',
            kind: PunctuationKind::CloseParen,
        }) {
            args.push(self.parse_expr());

            while self.match_type(&[&TokenType::Punctuation {
                raw: ',',
                kind: PunctuationKind::Comma,
            }]) {
                if args.len() >= 255 {
                    panic!("Can't have more than 255 arguments!");
                }
                args.push(self.parse_expr());
            }
        }

        let paren = self.consume_unit(
            &TokenType::Punctuation {
                raw: ')',
                kind: PunctuationKind::CloseParen,
            },
            "Expected ')' after argument(s)",
        );
        let expr = CallExpr {
            callee: Box::new(callee),
            paren,
            args,
        };
        Expr::Call(expr)
    }

    fn consume_unit(&mut self, token_type: &TokenType, message: &str) -> TokenType {
        if self.check_type(token_type) {
            return self.advance();
        }
        panic!("Invalid syntax, {}", message);
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

    fn match_look_ahead(&mut self, types: &[&TokenType]) -> bool {
        for token_type in types {
            if self.current + 1 >= self.tokens.len() {
                return false;
            } else {
                if &&self.tokens[self.current + 1] == token_type {
                    return true;
                }
            }
        }
        false
    }

    fn check_type(&mut self, token_type: &TokenType) -> bool {
        if self.end_of_stream() {
            return false;
        }
        &self.peek() == token_type
    }

    fn advance(&mut self) -> TokenType {
        if !self.end_of_stream() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> TokenType {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> TokenType {
        self.tokens[self.current - 1].clone()
    }

    fn end_of_stream(&self) -> bool {
        self.peek() == TokenType::EOF
    }
}
