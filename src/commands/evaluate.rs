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
        Expr::Grouping(expr) => evaluate(expr),
        Expr::Unary { prefix, expr } => match prefix {
            Operator::Bang => match evaluate(expr) {
                Literal::Boolean(false) => Literal::Boolean(true),
                Literal::Nil => Literal::Boolean(true),
                _ => Literal::Boolean(false),
            },
            Operator::Minus => match evaluate(expr) {
                Literal::Number(n) => {
                    let n: f64 = n.parse().unwrap();
                    Literal::Number(format!("{}", -n))
                }
                _ => todo!(),
            },
            _ => todo!(),
        },
        Expr::Binary { op, first, second } => match op {
            Operator::Star => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Number(format!("{}", a * b))
                }
                _ => todo!(),
            },
            Operator::Slash => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Number(format!("{}", a / b))
                }
                _ => todo!(),
            },
            Operator::Plus => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Number(format!("{}", a + b))
                }
                (Literal::String(a), Literal::String(b)) => Literal::String(format!("{}{}", a, b)),
                _ => todo!(),
            },
            Operator::Minus => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Number(format!("{}", a - b))
                }
                _ => todo!(),
            },
            Operator::Greater => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Boolean(a > b)
                }
                _ => todo!(),
            },
            Operator::Less => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Boolean(a < b)
                }
                _ => todo!(),
            },
            Operator::GreaterEqual => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Boolean(a >= b)
                }
                _ => todo!(),
            },
            Operator::LessEqual => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Boolean(a <= b)
                }
                _ => todo!(),
            },
            Operator::EqualEqual => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Boolean(a == b)
                }
                (Literal::Number(a), Literal::String(b)) => Literal::Boolean(false),
                (Literal::String(a), Literal::Number(b)) => Literal::Boolean(false),
                (Literal::String(a), Literal::String(b)) => Literal::Boolean(a == b),
                _ => todo!(),
            },
            Operator::BangEqual => match (evaluate(first), evaluate(second)) {
                (Literal::Number(a), Literal::Number(b)) => {
                    let a: f64 = a.parse().unwrap();
                    let b: f64 = b.parse().unwrap();
                    Literal::Boolean(a != b)
                }
                (Literal::String(a), Literal::String(b)) => Literal::Boolean(a != b),
                _ => todo!(),
            },
            _ => todo!(),
        },
    }
}
