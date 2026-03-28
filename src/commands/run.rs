use std::fs;

use crate::{
    commands::{parse::parse_statement, tokenize::tokenize},
    evaluator::Evaluator,
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
        let mut evaluator = Evaluator::new();
        while let Some(token) = iter.peek() {
            eprintln!("{}", token);
            match token {
                Token::Operator(op) => match op {
                    Operator::LeftBraces => {
                        iter.next();
                        evaluator.scope_in()
                    }
                    Operator::RightBraces => {
                        iter.next();
                        evaluator.scope_out()
                    }
                    _ => {}
                },
                t => {}
            }
            let stmt = parse_statement(iter);
            evaluator.execute(&stmt);
        }
    }
}
