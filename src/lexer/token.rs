use std::{fmt, str::FromStr};

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

impl FromStr for Token {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "and" => Ok(Token::And),
            "class" => Ok(Token::Class),
            "else" => Ok(Token::Else),
            "false" => Ok(Token::False),
            "for" => Ok(Token::For),
            "fun" => Ok(Token::Fun),
            "if" => Ok(Token::If),
            "nil" => Ok(Token::Nil),
            "or" => Ok(Token::Or),
            "print" => Ok(Token::Print),
            "return" => Ok(Token::Return),
            "super" => Ok(Token::Super),
            "this" => Ok(Token::This),
            "true" => Ok(Token::True),
            "var" => Ok(Token::Var),
            "while" => Ok(Token::While),
            _ => Err(s.to_string()),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParen => write!(f, "LEFT_PAREN ("),
            Token::RightParen => write!(f, "RIGHT_PAREN )"),
            Token::LeftBraces => write!(f, "LEFT_BRACE {{"),
            Token::RightBraces => write!(f, "RIGHT_BRACE }}"),
            Token::Star => write!(f, "STAR *"),
            Token::Dot => write!(f, "DOT ."),
            Token::Comma => write!(f, "COMMA ,"),
            Token::Plus => write!(f, "PLUS +"),
            Token::Minus => write!(f, "MINUS -"),
            Token::Semicolon => write!(f, "SEMICOLON ;"),
            Token::Slash => write!(f, "SLASH /"),
            Token::DoubleSlash => write!(f, "DOUBLE_SLASH //"),
            Token::Equal => write!(f, "EQUAL ="),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL =="),
            Token::Bang => write!(f, "BANG !"),
            Token::BangEqual => write!(f, "BANG_EQUAL !="),
            Token::Less => write!(f, "LESS <"),
            Token::LessEqual => write!(f, "LESS_EQUAL <="),
            Token::Greater => write!(f, "GREATER >"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >="),
            Token::StringQuote => write!(f, "STRING_QUOTE \""),
            Token::String(literal) => write!(f, "STRING \"{}\"", literal),
            Token::Number(literal) => write!(f, "NUMBER {}", literal),
            Token::Identifier(identifier) => write!(f, "IDENTIFIER {}", identifier),
            Token::And => write!(f, "AND and"),
            Token::Class => write!(f, "CLASS class"),
            Token::Else => write!(f, "ELSE else"),
            Token::False => write!(f, "FALSE false"),
            Token::For => write!(f, "FOR for"),
            Token::Fun => write!(f, "FUN fun"),
            Token::If => write!(f, "IF if"),
            Token::Nil => write!(f, "NIL nil"),
            Token::Or => write!(f, "OR or"),
            Token::Print => write!(f, "PRINT print"),
            Token::Return => write!(f, "RETURN return"),
            Token::Super => write!(f, "SUPER super"),
            Token::This => write!(f, "THIS this"),
            Token::True => write!(f, "TRUE true"),
            Token::Var => write!(f, "VAR var"),
            Token::While => write!(f, "WHILE while"),
            Token::EOF => write!(f, "EOF"),
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
            '0'..='9' => Ok(Token::Number("".to_string())),
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
