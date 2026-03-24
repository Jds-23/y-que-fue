use std::fs;
use std::iter::Peekable;

use crate::commands::tokenize::tokenize;
use crate::lexer::token::Token;
use crate::parser::expression::{Expr, Literal};

pub fn run(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read {}", filename);
        String::new()
    });
    if !file_contents.is_empty() {
        let tokens = tokenize(&file_contents).0;
        let expr: Expr = parse(&mut tokens.into_iter().peekable());
        println!("{}", expr)
        // for token in tokens {
        //     match token {
        //         Token::True => exprs.push(Expr::Literal(Literal::Boolean(true))),
        //         Token::False => exprs.push(Expr::Literal(Literal::Boolean(false))),
        //         Token::Nil => exprs.push(Expr::Literal(Literal::Nil)),
        //         Token::Number(s) => exprs.push(Expr::Literal(Literal::Number(s))),
        //         Token::String(s) => exprs.push(Expr::Literal(Literal::String(s))),
        //         _ => {}
        //     }
        // }
    }
}

pub fn parse(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    match iter.next() {
        Some(Token::True) => Expr::Literal(Literal::Boolean(true)),
        Some(Token::False) => Expr::Literal(Literal::Boolean(false)),
        Some(Token::Nil) => Expr::Literal(Literal::Nil),
        Some(Token::Number(s)) => Expr::Literal(Literal::Number(s)),
        Some(Token::String(s)) => Expr::Literal(Literal::String(s)),
        Some(Token::LeftParen) => {
            let group_expr = parse(iter);
            Expr::Grouping(Box::new(group_expr))
        }
        _ => todo!(),
    }
}
