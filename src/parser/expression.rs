use std::fmt;

use crate::{literal::Literal, operator::Operator};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary {
        prefix: Operator,
        expr: Box<Expr>,
    },
    Binary {
        op: Operator,
        first: Box<Expr>,
        second: Box<Expr>,
    },
    Identifier(String),
    Assign {
        name: String,
        value: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{}", literal.token_display()),
            Expr::Grouping(group) => write!(f, "(group {})", group),
            Expr::Unary { prefix, expr } => write!(f, "({} {})", prefix, expr),
            Expr::Binary {
                op, first, second, ..
            } => write!(f, "({} {} {})", op, first, second),
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::Assign { name, value } => write!(f, "(= {} {})", name, value),
        }
    }
}
