use std::{fmt, str::FromStr};

use crate::literal::Literal;
use crate::operator::Operator;

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Identifier(String),
    Literal(Literal),
    // Keywords
    And,
    Class,
    Else,
    For,
    Fun,
    If,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
    EOF,
}

impl FromStr for Token {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "and" => Ok(Token::And),
            "class" => Ok(Token::Class),
            "else" => Ok(Token::Else),
            "false" => Ok(Token::Literal(Literal::Boolean(false))),
            "for" => Ok(Token::For),
            "fun" => Ok(Token::Fun),
            "if" => Ok(Token::If),
            "nil" => Ok(Token::Literal(Literal::Nil)),
            "or" => Ok(Token::Or),
            "print" => Ok(Token::Print),
            "return" => Ok(Token::Return),
            "super" => Ok(Token::Super),
            "this" => Ok(Token::This),
            "true" => Ok(Token::Literal(Literal::Boolean(true))),
            "var" => Ok(Token::Var),
            "while" => Ok(Token::While),
            _ => Err(s.to_string()),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Operator(op) => match op {
                Operator::LeftParen => write!(f, "LEFT_PAREN ( null"),
                Operator::RightParen => write!(f, "RIGHT_PAREN ) null"),
                Operator::LeftBraces => write!(f, "LEFT_BRACE {{ null"),
                Operator::RightBraces => write!(f, "RIGHT_BRACE }} null"),
                Operator::Star => write!(f, "STAR * null"),
                Operator::Dot => write!(f, "DOT . null"),
                Operator::Comma => write!(f, "COMMA , null"),
                Operator::Plus => write!(f, "PLUS + null"),
                Operator::Minus => write!(f, "MINUS - null"),
                Operator::Semicolon => write!(f, "SEMICOLON ; null"),
                Operator::Slash => write!(f, "SLASH / null"),
                Operator::DoubleSlash => write!(f, "DOUBLE_SLASH //"),
                Operator::Equal => write!(f, "EQUAL = null"),
                Operator::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
                Operator::Bang => write!(f, "BANG ! null"),
                Operator::BangEqual => write!(f, "BANG_EQUAL != null"),
                Operator::Less => write!(f, "LESS < null"),
                Operator::LessEqual => write!(f, "LESS_EQUAL <= null"),
                Operator::Greater => write!(f, "GREATER > null"),
                Operator::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
                Operator::StringQuote => write!(f, "STRING_QUOTE \""),
            },
            Token::Literal(literal) => match literal {
                Literal::String(s) => write!(f, "STRING \"{}\" {}", s, literal),
                Literal::Number(n) => write!(f, "NUMBER {} {}", n, literal),
                Literal::Boolean(b) => match b {
                    true => write!(f, "TRUE {} null", literal),
                    false => write!(f, "FALSE {} null", literal),
                },
                Literal::Nil => write!(f, "NIL {} null", literal),
            },
            Token::Identifier(identifier) => write!(f, "IDENTIFIER {} null", identifier),
            Token::And => write!(f, "AND and null"),
            Token::Class => write!(f, "CLASS class null"),
            Token::Else => write!(f, "ELSE else null"),
            Token::For => write!(f, "FOR for null"),
            Token::Fun => write!(f, "FUN fun null"),
            Token::If => write!(f, "IF if null"),
            Token::Or => write!(f, "OR or null"),
            Token::Print => write!(f, "PRINT print null"),
            Token::Return => write!(f, "RETURN return null"),
            Token::Super => write!(f, "SUPER super null"),
            Token::This => write!(f, "THIS this null"),
            Token::Var => write!(f, "VAR var null"),
            Token::While => write!(f, "WHILE while null"),
            Token::EOF => write!(f, "EOF  null"),
        }
    }
}

impl Token {
    pub fn from_char(t: &char) -> Result<Token, char> {
        match t {
            '0'..='9' => Ok(Token::Literal(Literal::Number("".to_string()))),
            _ => Operator::from_char(t).map(Token::Operator),
        }
    }
    pub fn double_char_operator(&self, next: char) -> Option<Token> {
        match self {
            Token::Operator(op) => op.double_char(next).map(Token::Operator),
            _ => None,
        }
    }
}
