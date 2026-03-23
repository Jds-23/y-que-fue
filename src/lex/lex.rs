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
    EOF
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        match self {
            Tokens::LeftParen=>write!(f,"LEFT_PAREN ("),
            Tokens::RightParen=>write!(f,"RIGHT_PAREN )"),
            Tokens::LeftBraces=>write!(f,"LEFT_BRACE {{"),
            Tokens::RightBraces=>write!(f,"RIGHT_BRACE }}"),
            Tokens::Star=>write!(f,"STAR *"),
            Tokens::Dot=>write!(f,"DOT ."),
            Tokens::Comma=>write!(f,"COMMA ,"),
            Tokens::Plus=>write!(f,"PLUS +"),
            Tokens::EOF=>write!(f,"EOF")
        }
    }
}

impl FromStr for Tokens {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self,Self::Err> {
        match s {
            "("=>Ok(Tokens::LeftParen),
            ")"=>Ok(Tokens::RightParen),
            "{"=>Ok(Tokens::LeftBraces),
            "}"=>Ok(Tokens::RightBraces),
            "*"=>Ok(Tokens::Star),
            "."=>Ok(Tokens::Dot),
            ","=>Ok(Tokens::Comma),
            "+"=>Ok(Tokens::Plus),
            _=>Err(s.to_string()),
        }
    }
}
