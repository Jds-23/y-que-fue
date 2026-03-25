#![allow(unused_variables)]
use std::env;

mod commands;
mod lexer;
mod literal;
mod operator;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => commands::tokenize::run(filename),
        "parse" => commands::parse::run(filename),
        "evaluate" => commands::evaluate::run(filename),
        "run" => commands::run::run(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
