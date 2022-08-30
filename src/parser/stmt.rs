use crate::lexer::Token;
use super::ast::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Let{ token: Token, initilizer: Expr }
}