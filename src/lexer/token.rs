use std::{fmt, str::FromStr};

use crate::literal::Literal;

#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBraces,
    RightBraces,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Slash,
    DoubleSlash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    StringQuote,
    Identifier(String),
    // Literals
    // Number(String),
    // String(String),
    // Nil,
    // False,
    // True,
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
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBraces => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBraces => write!(f, "RIGHT_BRACE }} null"),
            Token::Star => write!(f, "STAR * null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::DoubleSlash => write!(f, "DOUBLE_SLASH //"),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Token::Bang => write!(f, "BANG ! null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::Less => write!(f, "LESS < null"),
            Token::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::StringQuote => write!(f, "STRING_QUOTE \""),
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
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBraces),
            '}' => Ok(Token::RightBraces),
            '*' => Ok(Token::Star),
            '.' => Ok(Token::Dot),
            ',' => Ok(Token::Comma),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            ';' => Ok(Token::Semicolon),
            '/' => Ok(Token::Slash),
            '=' => Ok(Token::Equal),
            '!' => Ok(Token::Bang),
            '<' => Ok(Token::Less),
            '>' => Ok(Token::Greater),
            '0'..='9' => Ok(Token::Literal(Literal::Number("".to_string()))),
            '"' => Ok(Token::StringQuote),
            _ => Err(*t),
        }
    }
    pub fn double_char_operator(&self, next: char) -> Option<Token> {
        match self {
            Token::Equal => match next {
                '=' => Some(Token::EqualEqual),
                _ => None,
            },
            Token::Bang => match next {
                '=' => Some(Token::BangEqual),
                _ => None,
            },
            Token::Less => match next {
                '=' => Some(Token::LessEqual),
                _ => None,
            },
            Token::Greater => match next {
                '=' => Some(Token::GreaterEqual),
                _ => None,
            },
            Token::Slash => match next {
                '/' => Some(Token::DoubleSlash),
                _ => None,
            },
            _ => None,
        }
    }
}
