#![allow(unused_variables)]
use std::env;
use std::fs;
use std::str::FromStr;

mod lex;

use crate::lex::lex::Tokens;

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
                    match Tokens::from_str(&token.to_string()) {
                        Ok(Tokens::StringQuote) => {
                            let mut string_literal: Vec<char> = vec![];
                            let mut terminated = false;
                            while let Some(t) = iter.next() {
                                if t == '\n' {
                                    line += 1;
                                }
                                if t == '\"' {
                                    terminated = true;
                                    break;
                                }
                                string_literal.push(t);
                            }
                            if !terminated {
                                eprintln!("[line {}] Error: Unterminated string.", line);
                                std::process::exit(65);
                            } else {
                                let s: String = string_literal.iter().collect();
                                println!("{} {}", Tokens::String(s.clone()), s);
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
            println!("{}  null", Tokens::EOF); // Placeholder, replace this line when implementing the scanner

            if has_lexical_errors {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
