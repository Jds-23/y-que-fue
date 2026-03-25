use std::fs;

use crate::commands::parse::parse;
use crate::commands::tokenize::tokenize;
use crate::literal::Literal;
use crate::operator::Operator;
use crate::parser::expression::Expr;

pub fn run(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read {}", filename);
        String::new()
    });
    if !file_contents.is_empty() {
        let tokens = tokenize(&file_contents).0;
        let expr = parse(&mut tokens.into_iter().peekable());
        match evaluate(&expr) {
            Literal::Number(n) => {
                println!("{}", n);
            }
            literal => {
                println!("{}", literal)
            }
        };
    }
}

pub fn evaluate(expr: &Expr) -> Literal {
    match expr {
        Expr::Literal(literal) => literal.clone(),
        // Expr::Literal(literal) => match literal {
        //     Literal::Number(n) => format!("{}", n),
        //     _ => format!("{}", literal),
        // },
        Expr::Grouping(expr) => evaluate(expr),
        Expr::Unary { prefix, expr } => match prefix {
            Operator::Bang => match evaluate(expr) {
                Literal::Boolean(false) => Literal::Boolean(true),
                Literal::Nil => Literal::Boolean(true),
                _ => Literal::Boolean(false),
            },
            Operator::Minus => match **expr {
                Expr::Literal(ref literal) => match literal {
                    Literal::Number(n) => Literal::Number(format!("-{}", n)),
                    _ => todo!(),
                },
                _ => todo!(),
            },
            _ => todo!(),
        },
        _ => todo!(),
    }
}
