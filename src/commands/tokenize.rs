use std::fs;
use std::str::FromStr;

use crate::lexer::error::LexError;
use crate::lexer::identifier::extract_identifier;
use crate::lexer::number::extract_number_literal;
use crate::lexer::string::extract_string_literal;
use crate::lexer::token::Token;
use crate::literal::Literal;

pub fn run(filename: &str) {
    eprintln!("Logs from your program will appear here!");

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    let (tokens, errors) = tokenize(&file_contents);

    // Print errors to stderr
    for err in &errors {
        match err {
            LexError::UnterminatedString { line } => {
                eprintln!("[line {}] Error: Unterminated string.", line);
            }
            LexError::UnexpectedCharacter { line, ch } => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, ch);
            }
        }
    }

    // Print all tokens
    for t in &tokens {
        println!("{}", t);
    }

    println!("{}", Token::EOF);

    if !errors.is_empty() {
        std::process::exit(65);
    }
}

pub fn tokenize(file_contents: &str) -> (Vec<Token>, Vec<LexError>) {
    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<LexError> = vec![];

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
            }
            if token.is_alphabetic() || token == '_' {
                let i = extract_identifier(&mut iter, &token);
                let token = Token::from_str(&i);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(s) => tokens.push(Token::Identifier(s)),
                }
                continue;
            }
            match Token::from_char(&token) {
                Ok(Token::StringQuote) => {
                    let result = extract_string_literal(&mut iter, &mut line);
                    match result {
                        Ok(s) => tokens.push(Token::Literal(Literal::String(s))),
                        Err(_) => {
                            errors.push(LexError::UnterminatedString { line });
                            break;
                        }
                    }
                }
                Ok(Token::Literal(Literal::Number(_))) => {
                    let s = extract_number_literal(&mut iter, &token);
                    match s {
                        Some(s) => tokens.push(Token::Literal(Literal::Number(s))),
                        None => {
                            errors.push(LexError::UnexpectedCharacter { line, ch: token });
                            break;
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
                            tokens.push(t);
                        }
                        None => tokens.push(t),
                    }
                }
                Err(e) => {
                    errors.push(LexError::UnexpectedCharacter { line, ch: e });
                }
            }
        }
    }

    (tokens, errors)
}
