use std::fs;

use crate::commands::parse::parse;
use crate::commands::tokenize::tokenize;
use crate::parser::expression::Expr;

pub fn run(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read {}", filename);
        String::new()
    });
    if !file_contents.is_empty() {
        let tokens = tokenize(&file_contents).0;
        let expr = parse(&mut tokens.into_iter().peekable());
        println!("{}", evaluate(&expr))
    }
}

pub fn evaluate(expr: &Expr) -> String {
    match expr {
        Expr::Literal(literal) => format!("{}", literal),
        _ => todo!(),
    }
}
