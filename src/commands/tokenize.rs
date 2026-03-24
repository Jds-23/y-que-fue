use std::fs;
use std::str::FromStr;

use crate::lexer::identifier::extract_identifier;
use crate::lexer::number::extract_number_literal;
use crate::lexer::string::extract_string_literal;
use crate::lexer::token::Token;

pub fn run(filename: &str) {
    eprintln!("Logs from your program will appear here!");

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    let mut has_lexical_errors = false;
    if !file_contents.is_empty() {
        let mut iter = file_contents.chars().peekable();
        let mut line = 1;
        while let Some(token) = iter.next() {
            if token == '\n' {
                line += 1;
                continue;
            }
            if token.is_whitespace() {
                continue;
            };
            if token.is_alphabetic() || token == '_' {
                let i = extract_identifier(&mut iter, &token);
                let token = Token::from_str(&i);
                match token {
                    Ok(t) => println!("{} null", t),
                    Err(s) => println!("{} null", Token::Identifier(s)),
                }
                continue;
            }
            match Token::from_char(&token) {
                Ok(Token::StringQuote) => {
                    let result = extract_string_literal(&mut iter, &mut line);
                    match result {
                        Ok(s) => {
                            println!("{} {}", Token::String(s.clone()), s);
                        }
                        Err(_) => {
                            eprintln!("[line {}] Error: Unterminated string.", line);
                            println!("{}  null", Token::EOF);
                            std::process::exit(65);
                        }
                    }
                }
                Ok(Token::Number(_)) => {
                    let s = extract_number_literal(&mut iter, &token);
                    match s {
                        Some(s) => {
                            let n: f64 = s.parse().unwrap();
                            let out = if n.fract() == 0.0 {
                                format!("{:.1}", n)
                            } else {
                                format!("{}", n)
                            };
                            println!("{} {}", Token::Number(s), out);
                        }
                        None => {
                            eprintln!("[line {}] Error: Unexpected character.", line);
                            std::process::exit(65);
                        }
                    }
                }
                Ok(t) => {
                    let next = iter.peek().copied().unwrap_or('\0');
                    match t.double_char_operator(next) {
                        Some(Token::DoubleSlash) => {
                            while let Some(t) = iter.next() {
                                if t == '\n' {
                                    line += 1;
                                    break;
                                }
                            }
                        }
                        Some(t) => {
                            iter.next();
                            println!("{} null", t)
                        }
                        None => println!("{} null", t),
                    }
                }
                Err(e) => {
                    has_lexical_errors = true;
                    eprintln!("[line {}] Error: Unexpected character: {}", line, e);
                }
            }
        }
    }
    println!("{}  null", Token::EOF);

    if has_lexical_errors {
        std::process::exit(65);
    }
}
