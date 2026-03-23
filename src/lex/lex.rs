use std::{fmt, str::FromStr};

#[derive(Debug)]
pub enum Tokens {
    LeftParen,
    RightParen,
    EOF
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        match self {
            Tokens::LeftParen=>write!(f,"LEFT_PAREN ("),
            Tokens::RightParen=>write!(f,"RIGHT_PAREN )"),
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
            _=>Err(s.to_string()),
        }
    }
}
