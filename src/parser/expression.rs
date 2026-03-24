use std::fmt;

use crate::{lexer::token::Token, literal::Literal};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary { prefix: Token, expr: Box<Expr> },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Grouping(group) => write!(f, "(group {})", group),
            Expr::Unary {
                prefix: Token::Minus,
                expr,
            } => write!(f, "(- {})", expr),
            Expr::Unary {
                prefix: Token::Bang,
                expr,
            } => write!(f, "(! {})", expr),
            _ => todo!(),
        }
    }
}
