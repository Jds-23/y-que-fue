use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Operator {
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
}

impl Operator {
    pub fn from_char(c: &char) -> Result<Operator, char> {
        match c {
            '(' => Ok(Operator::LeftParen),
            ')' => Ok(Operator::RightParen),
            '{' => Ok(Operator::LeftBraces),
            '}' => Ok(Operator::RightBraces),
            '*' => Ok(Operator::Star),
            '.' => Ok(Operator::Dot),
            ',' => Ok(Operator::Comma),
            '+' => Ok(Operator::Plus),
            '-' => Ok(Operator::Minus),
            ';' => Ok(Operator::Semicolon),
            '/' => Ok(Operator::Slash),
            '=' => Ok(Operator::Equal),
            '!' => Ok(Operator::Bang),
            '<' => Ok(Operator::Less),
            '>' => Ok(Operator::Greater),
            '"' => Ok(Operator::StringQuote),
            _ => Err(*c),
        }
    }

    pub fn double_char(&self, next: char) -> Option<Operator> {
        match self {
            Operator::Equal => match next {
                '=' => Some(Operator::EqualEqual),
                _ => None,
            },
            Operator::Bang => match next {
                '=' => Some(Operator::BangEqual),
                _ => None,
            },
            Operator::Less => match next {
                '=' => Some(Operator::LessEqual),
                _ => None,
            },
            Operator::Greater => match next {
                '=' => Some(Operator::GreaterEqual),
                _ => None,
            },
            Operator::Slash => match next {
                '/' => Some(Operator::DoubleSlash),
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::LeftParen => write!(f, "("),
            Operator::RightParen => write!(f, ")"),
            Operator::LeftBraces => write!(f, "{{"),
            Operator::RightBraces => write!(f, "}}"),
            Operator::Star => write!(f, "*"),
            Operator::Dot => write!(f, "."),
            Operator::Comma => write!(f, ","),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Semicolon => write!(f, ";"),
            Operator::Slash => write!(f, "/"),
            Operator::DoubleSlash => write!(f, "//"),
            Operator::Equal => write!(f, "="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::Bang => write!(f, "!"),
            Operator::BangEqual => write!(f, "!="),
            Operator::Less => write!(f, "<"),
            Operator::LessEqual => write!(f, "<="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::StringQuote => write!(f, "\""),
        }
    }
}
