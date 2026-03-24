#![allow(unused_variables)]
use std::env;
use std::fs;
use std::str::FromStr;

mod lex;

use crate::lex::identifier::extract_identifier;
use crate::lex::lex::Tokens;
use crate::lex::number::extract_number_literal;
use crate::lex::string::extract_string_literal;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut has_lexical_errors = false;
            // TODO: Uncomment the code below to pass the first stage
            if !file_contents.is_empty() {
                // let tokens: Vec<&str> = file_contents.split("").collect();
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
                        let token = Tokens::from_str(&i);
                        match token {
                            Ok(t) => println!("{} null", t),
                            Err(s) => println!("{} null", Tokens::Identifier(s)),
                        }
                        continue;
                    }
                    match Tokens::from_char(&token) {
                        Ok(Tokens::StringQuote) => {
                            let result = extract_string_literal(&mut iter, &mut line);
                            match result {
                                Ok(s) => {
                                    println!("{} {}", Tokens::String(s.clone()), s);
                                }
                                Err(_) => {
                                    eprintln!("[line {}] Error: Unterminated string.", line);
                                    println!("{}  null", Tokens::EOF);
                                    std::process::exit(65);
                                }
                            }
                        }
                        Ok(Tokens::Number(_)) => {
                            let s = extract_number_literal(&mut iter, &token);
                            match s {
                                Some(s) => {
                                    let n: f64 = s.parse().unwrap();
                                    let out = if n.fract() == 0.0 {
                                        format!("{:.1}", n) // 3.0
                                    } else {
                                        format!("{}", n) // 3.14 (keeps all decimals)
                                    };
                                    println!("{} {}", Tokens::Number(s), out);
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
                                Some(Tokens::DoubleSlash) => {
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
            println!("{}  null", Tokens::EOF);

            if has_lexical_errors {
                std::process::exit(65);
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Unable to read {}", filename);
                String::new()
            });
            if !file_contents.is_empty() {
                // println!("{}", file_contents);
                match Tokens::from_str(&file_contents) {
                    Ok(Tokens::True) => println!("{}", true),
                    Ok(Tokens::False) => println!("{}", false),
                    Ok(Tokens::Nil) => println!("nil"),
                    Err(s) => {
                        let n: f64 = s.parse().unwrap();
                        let out = if n.fract() == 0.0 {
                            format!("{:.1}", n) // 3.0
                        } else {
                            format!("{}", n) // 3.14 (keeps all decimals)
                        };
                        println!("{}", out);
                    }
                    _ => {}
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
