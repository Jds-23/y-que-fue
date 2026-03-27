use std::fs;

use crate::{
    commands::{parse::parse, tokenize::tokenize},
    evaluator::Evaluator,
    lexer::token::Token,
    literal::Literal,
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
        let mut evaluator = Evaluator::new();
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
                    println!("{}", evaluator.evaluate(&expr));
                }
                Token::Var => {
                    iter.next();
                    match iter.peek() {
                        Some(Token::Identifier(identifier)) => {
                            let identifier = identifier.clone();
                            iter.next();
                            match iter.next() {
                                Some(Token::Operator(Operator::Equal)) => {
                                    let expr = parse(iter);
                                    match iter.next() {
                                        Some(Token::Operator(Operator::Semicolon)) => {}
                                        _ => {
                                            eprintln!("Expect ';' after statement.");
                                            std::process::exit(65);
                                        }
                                    }
                                    let val = evaluator.evaluate(&expr);
                                    evaluator.insert(identifier, val);
                                }
                                Some(Token::Operator(Operator::Semicolon)) => {
                                    evaluator.insert(identifier, Literal::Nil);
                                }
                                _ => todo!(),
                            };
                        }
                        _ => {
                            eprintln!("Expected identifer after var.");
                            std::process::exit(65);
                        }
                    }
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
                    evaluator.evaluate(&expr);
                }
            }
        }
    }
}
