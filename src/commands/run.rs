use std::fs;

use crate::{
    commands::{evaluate::evaluate, parse::parse, tokenize::tokenize},
    lexer::token::Token,
    operator::Operator,
};

pub fn run(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read {}", filename);
        String::new()
    });
    if !file_contents.is_empty() {
        let tokens = tokenize(&file_contents).0;
        let iter = &mut tokens.into_iter().peekable();
        while let Some(token) = iter.peek() {
            match token {
                Token::Print => {
                    iter.next();
                    let expr = parse(iter);
                    match iter.next() {
                        Some(Token::Operator(Operator::Semicolon)) => {}
                        _ => {
                            eprintln!("Expect ';' after statement.");
                            std::process::exit(65);
                        }
                    }
                    println!("{}", evaluate(&expr));
                }
                _ => {
                    let expr = parse(iter);
                    match iter.next() {
                        Some(Token::Operator(Operator::Semicolon)) => {}
                        Some(_) => {
                            std::process::exit(70);
                        }
                        _ => todo!(),
                    }
                    evaluate(&expr);
                }
            }
        }
    }
}
