use std::fs;

use crate::commands::tokenize::tokenize;
use crate::lexer::token::Token;

pub fn run(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read {}", filename);
        String::new()
    });
    if !file_contents.is_empty() {
        let tokens = tokenize(&file_contents).0;
        for token in tokens {
            match token {
                Token::True => println!("{}", true),
                Token::False => println!("{}", false),
                Token::Nil => println!("nil"),
                Token::Number(s) => {
                    let n: f64 = s.parse().unwrap();
                    let out = if n.fract() == 0.0 {
                        format!("{:.1}", n)
                    } else {
                        format!("{}", n)
                    };
                    println!("{}", out);
                }
                _ => {}
            }
        }
    }
}
