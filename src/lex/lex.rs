use std::fmt;

#[derive(Debug)]
pub enum Tokens {
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
    String(String),
    Number(String),
    EOF,
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tokens::LeftParen => write!(f, "LEFT_PAREN ("),
            Tokens::RightParen => write!(f, "RIGHT_PAREN )"),
            Tokens::LeftBraces => write!(f, "LEFT_BRACE {{"),
            Tokens::RightBraces => write!(f, "RIGHT_BRACE }}"),
            Tokens::Star => write!(f, "STAR *"),
            Tokens::Dot => write!(f, "DOT ."),
            Tokens::Comma => write!(f, "COMMA ,"),
            Tokens::Plus => write!(f, "PLUS +"),
            Tokens::Minus => write!(f, "MINUS -"),
            Tokens::Semicolon => write!(f, "SEMICOLON ;"),
            Tokens::Slash => write!(f, "SLASH /"),
            Tokens::DoubleSlash => write!(f, "DOUBLE_SLASH //"),
            Tokens::Equal => write!(f, "EQUAL ="),
            Tokens::EqualEqual => write!(f, "EQUAL_EQUAL =="),
            Tokens::Bang => write!(f, "BANG !"),
            Tokens::BangEqual => write!(f, "BANG_EQUAL !="),
            Tokens::Less => write!(f, "LESS <"),
            Tokens::LessEqual => write!(f, "LESS_EQUAL <="),
            Tokens::Greater => write!(f, "GREATER >"),
            Tokens::GreaterEqual => write!(f, "GREATER_EQUAL >="),
            Tokens::StringQuote => write!(f, "STRING_QUOTE \""),
            Tokens::String(literal) => write!(f, "STRING \"{}\"", literal),
            Tokens::Number(literal) => write!(f, "NUMBER {}", literal),
            Tokens::EOF => write!(f, "EOF"),
        }
    }
}

impl Tokens {
    pub fn from_char(t: &char) -> Result<Tokens, char> {
        match t {
            '(' => Ok(Tokens::LeftParen),
            ')' => Ok(Tokens::RightParen),
            '{' => Ok(Tokens::LeftBraces),
            '}' => Ok(Tokens::RightBraces),
            '*' => Ok(Tokens::Star),
            '.' => Ok(Tokens::Dot),
            ',' => Ok(Tokens::Comma),
            '+' => Ok(Tokens::Plus),
            '-' => Ok(Tokens::Minus),
            ';' => Ok(Tokens::Semicolon),
            '/' => Ok(Tokens::Slash),
            // '//' => Ok(Tokens::DoubleSlash),
            '=' => Ok(Tokens::Equal),
            // '==' => Ok(Tokens::EqualEqual),
            '!' => Ok(Tokens::Bang),
            // '!=' => Ok(Tokens::BangEqual),
            '<' => Ok(Tokens::Less),
            // '<=' => Ok(Tokens::LessEqual),
            '>' => Ok(Tokens::Greater),
            '0'..='9' => Ok(Tokens::Number("".to_string())),
            // '>=' => Ok(Tokens::GreaterEqual),
            '"' => Ok(Tokens::StringQuote),
            _ => Err(*t),
        }
    }
    pub fn double_char_operator(&self, next: char) -> Option<Tokens> {
        match self {
            Tokens::Equal => match next {
                '=' => Some(Tokens::EqualEqual),
                _ => None,
            },
            Tokens::Bang => match next {
                '=' => Some(Tokens::BangEqual),
                _ => None,
            },
            Tokens::Less => match next {
                '=' => Some(Tokens::LessEqual),
                _ => None,
            },
            Tokens::Greater => match next {
                '=' => Some(Tokens::GreaterEqual),
                _ => None,
            },
            Tokens::Slash => match next {
                '/' => Some(Tokens::DoubleSlash),
                _ => None,
            },
            _ => None,
        }
    }
}
