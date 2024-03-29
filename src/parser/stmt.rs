use super::expr::Expr;
use crate::lexer::TokenType;

#[derive(Debug)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression(Expr),
    Print(Expr),
    Let {
        token: TokenType,
        initilizer: Expr,
    },
    If {
        condition: Expr,
        branch: Box<Stmt>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: TokenType,
        params: Vec<TokenType>,
        body: Vec<Stmt>,
    },
    Return {
        keyword: TokenType,
        value: Option<Expr>,
    },
    Class {
        name: TokenType,
        super_class: Option<Expr>,
        methods: Vec<Stmt>,
    },
}
