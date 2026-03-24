use std::fs;
use std::str::FromStr;

use crate::lexer::token::Token;

pub fn run(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read {}", filename);
        String::new()
    });
    if !file_contents.is_empty() {
        match Token::from_str(&file_contents) {
            Ok(Token::True) => println!("{}", true),
            Ok(Token::False) => println!("{}", false),
            Ok(Token::Nil) => println!("nil"),
            Err(s) => {
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
