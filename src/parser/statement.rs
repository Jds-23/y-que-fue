use std::fmt;

use crate::{commands::evaluate::evaluate, parser::expression::Expr};

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Print(expr) => write!(f, "{}", evaluate(&expr)),
            _ => todo!(),
        }
    }
}
