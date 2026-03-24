use std::fs;
use std::iter::Peekable;

use crate::commands::tokenize::tokenize;
use crate::lexer::token::Token;
use crate::parser::expression::Expr;

pub fn run(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read {}", filename);
        String::new()
    });
    if !file_contents.is_empty() {
        let tokens = tokenize(&file_contents).0;
        let expr: Expr = parse(&mut tokens.into_iter().peekable());
        println!("{}", expr)
    }
}

pub fn parse(iter: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    let expr = match iter.next() {
        Some(Token::Literal(l)) => Expr::Literal(l),
        Some(Token::LeftParen) => {
            let group_expr = parse(iter);
            Expr::Grouping(Box::new(group_expr))
        }
        Some(Token::Minus) => {
            let group_expr = parse(iter);
            Expr::Unary {
                expr: Box::new(group_expr),
                prefix: Token::Minus,
            }
        }
        Some(Token::Bang) => {
            let group_expr = parse(iter);
            Expr::Unary {
                expr: Box::new(group_expr),
                prefix: Token::Bang,
            }
        }
        _ => todo!(),
    };
    match iter.peek() {
        Some(Token::Star) => {
            iter.next();
            Expr::Binary {
                op: Token::Star,
                first: Box::new(expr),
                second: Box::new(parse(iter)),
            }
        }
        Some(Token::Slash) => {
            iter.next();
            Expr::Binary {
                op: Token::Slash,
                first: Box::new(expr),
                second: Box::new(parse(iter)),
            }
        }
        _ => expr,
    }
}
