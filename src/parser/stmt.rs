use crate::lexer::TokenType;
use super::ast::Expr;

#[derive(Debug)]
pub enum Stmt {
    Block{ statements: Vec<Stmt> },
    Expression(Expr),
    Print(Expr),
    Let{ token: TokenType, initilizer: Expr }
}