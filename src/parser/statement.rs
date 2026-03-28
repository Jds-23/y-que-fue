use crate::parser::expression::Expr;

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    Expression(Expr),
}
