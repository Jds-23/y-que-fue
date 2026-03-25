use std::{fs, iter::Peekable};

use crate::{
    commands::{parse::parse, tokenize::tokenize},
    lexer::token::Token,
    operator::Operator,
    parser::statement::Stmt,
};

pub fn run(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read {}", filename);
        String::new()
    });
    if !file_contents.is_empty() {
        let tokens = tokenize(&file_contents).0;
        let mut stmts = split_into_exprs(&mut tokens.into_iter().peekable()).into_iter();
        while let Some(stmt) = stmts.next() {
            println!("{}", stmt);
        }
    }
}

pub fn split_into_exprs(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Vec<Stmt> {
    let mut exprs: Vec<Stmt> = vec![];
    while let Some(token) = iter.peek() {
        match token {
            Token::Print => {
                iter.next();
                exprs.push(Stmt::Print(parse(iter)));
                match iter.next() {
                    Some(Token::Operator(Operator::Semicolon)) => {}
                    _ => {
                        eprintln!("Expect ';' after statement.");
                        std::process::exit(65);
                    }
                }
            }
            _ => {
                // exprs.push(Stmt::Print(parse(iter)));
                parse(iter);
                match iter.next() {
                    Some(Token::Operator(Operator::Semicolon)) => {}
                    Some(token) => {
                        todo!("{}", token)
                        // eprintln!("Expect ';' after statement.");
                        // std::process::exit(65);
                    }
                    _ => todo!(),
                }
            }
        }
    }
    exprs
}
