use std::fmt;

use crate::{lexer::token::Token, literal::Literal};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary {
        prefix: Token,
        expr: Box<Expr>,
    },
    Binary {
        op: Token,
        first: Box<Expr>,
        second: Box<Expr>,
    },
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
            Expr::Binary {
                op: Token::Star,
                first,
                second,
            } => write!(f, "(* {} {})", first, second),
            Expr::Binary {
                op: Token::Slash,
                first,
                second,
            } => write!(f, "(/ {} {})", first, second),
            Expr::Binary {
                op: Token::Minus,
                first,
                second,
            } => write!(f, "(- {} {})", first, second),
            Expr::Binary {
                op: Token::Plus,
                first,
                second,
            } => write!(f, "(+ {} {})", first, second),
            _ => todo!(),
        }
    }
}
