use std::{fmt, str::FromStr};

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
    Identifier(String),
    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

impl FromStr for Tokens {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "and" => Ok(Tokens::And),
            "class" => Ok(Tokens::Class),
            "else" => Ok(Tokens::Else),
            "false" => Ok(Tokens::False),
            "for" => Ok(Tokens::For),
            "fun" => Ok(Tokens::Fun),
            "if" => Ok(Tokens::If),
            "nil" => Ok(Tokens::Nil),
            "or" => Ok(Tokens::Or),
            "print" => Ok(Tokens::Print),
            "return" => Ok(Tokens::Return),
            "super" => Ok(Tokens::Super),
            "this" => Ok(Tokens::This),
            "true" => Ok(Tokens::True),
            "var" => Ok(Tokens::Var),
            "while" => Ok(Tokens::While),
            _ => Err(s.to_string()),
        }
    }
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
            Tokens::Identifier(identifier) => write!(f, "IDENTIFIER {}", identifier),
            Tokens::And => write!(f, "AND and"),
            Tokens::Class => write!(f, "CLASS class"),
            Tokens::Else => write!(f, "ELSE else"),
            Tokens::False => write!(f, "FALSE false"),
            Tokens::For => write!(f, "FOR for"),
            Tokens::Fun => write!(f, "FUN fun"),
            Tokens::If => write!(f, "IF if"),
            Tokens::Nil => write!(f, "NIL nil"),
            Tokens::Or => write!(f, "OR or"),
            Tokens::Print => write!(f, "PRINT print"),
            Tokens::Return => write!(f, "RETURN return"),
            Tokens::Super => write!(f, "SUPER super"),
            Tokens::This => write!(f, "THIS this"),
            Tokens::True => write!(f, "TRUE true"),
            Tokens::Var => write!(f, "VAR var"),
            Tokens::While => write!(f, "WHILE while"),
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
