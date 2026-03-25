use std::fs;
use std::iter::Peekable;

use crate::commands::tokenize::tokenize;
use crate::lexer::token::Token;
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
        println!("{}", expr)
    }
}

pub fn parse(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    parse_equality(iter)
}

fn parse_equality(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    let mut expr = parse_comparison(iter);
    loop {
        match iter.peek() {
            Some(Token::Operator(Operator::EqualEqual))
            | Some(Token::Operator(Operator::BangEqual)) => {
                let op = match iter.next().unwrap() {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                };
                let right = parse_comparison(iter);
                expr = Expr::Binary {
                    op,
                    first: Box::new(expr),
                    second: Box::new(right),
                };
            }
            _ => break,
        }
    }
    expr
}
fn parse_comparison(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    let mut expr = parse_addition_subtraction(iter);
    loop {
        match iter.peek() {
            Some(Token::Operator(Operator::Greater))
            | Some(Token::Operator(Operator::Less))
            | Some(Token::Operator(Operator::GreaterEqual))
            | Some(Token::Operator(Operator::LessEqual)) => {
                let op = match iter.next().unwrap() {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                };
                let right = parse_addition_subtraction(iter);
                expr = Expr::Binary {
                    op,
                    first: Box::new(expr),
                    second: Box::new(right),
                };
            }
            _ => break,
        }
    }
    expr
}

fn parse_addition_subtraction(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    let mut expr = parse_multiplicative(iter);
    loop {
        match iter.peek() {
            Some(Token::Operator(Operator::Minus)) | Some(Token::Operator(Operator::Plus)) => {
                let op = match iter.next().unwrap() {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                };
                let right = parse_multiplicative(iter);
                expr = Expr::Binary {
                    op,
                    first: Box::new(expr),
                    second: Box::new(right),
                };
            }
            _ => break,
        }
    }
    expr
}

fn parse_multiplicative(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    let mut expr = parse_primary(iter);
    loop {
        match iter.peek() {
            Some(Token::Operator(Operator::Star)) | Some(Token::Operator(Operator::Slash)) => {
                let op = match iter.next().unwrap() {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                };
                let right = parse_primary(iter);
                expr = Expr::Binary {
                    op,
                    first: Box::new(expr),
                    second: Box::new(right),
                };
            }
            _ => break,
        }
    }
    expr
}

fn parse_primary(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    match iter.next() {
        Some(Token::Literal(l)) => Expr::Literal(l),
        Some(Token::Operator(Operator::LeftParen)) => {
            let group_expr = parse(iter);
            match iter.next() {
                Some(Token::Operator(Operator::RightParen)) => Expr::Grouping(Box::new(group_expr)),
                _ => {
                    eprintln!("Expect ')' after expression.");
                    std::process::exit(65);
                }
            }
        }
        Some(Token::Operator(op @ Operator::Minus)) => {
            let right = parse_primary(iter);
            Expr::Unary {
                expr: Box::new(right),
                prefix: op,
            }
        }
        Some(Token::Operator(op @ Operator::Bang)) => {
            let right = parse_primary(iter);
            Expr::Unary {
                expr: Box::new(right),
                prefix: op,
            }
        }
        Some(t) => {
            eprintln!("Unexpected '{}' after expression.", t);
            std::process::exit(65);
        }
        _ => {
            eprintln!("Unexpected expression.");
            std::process::exit(65);
        }
    }
}
