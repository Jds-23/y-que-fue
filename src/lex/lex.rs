use std::{fmt, str::FromStr};

#[derive(Debug)]
pub enum Tokens {
    LeftParen,
    RightParen,
    LeftBraces,
    RightBraces,
    EOF
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        match self {
            Tokens::LeftParen=>write!(f,"LEFT_PAREN ("),
            Tokens::RightParen=>write!(f,"RIGHT_PAREN )"),
            Tokens::LeftBraces=>write!(f,"LEFT_BRACE {{"),
            Tokens::RightBraces=>write!(f,"RIGHT_BRACE }}"),
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
            _=>Err(s.to_string()),
        }
    }
}
