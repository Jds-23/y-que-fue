use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(bool),
    Nil,
}

impl Literal {
    pub fn token_display(&self) -> String {
        match self {
            Literal::Number(s) => {
                let n: f64 = s.parse().unwrap();
                if n.fract() == 0.0 {
                    format!("{:.1}", n)
                } else {
                    format!("{}", n)
                }
            }
            _ => format!("{}", self),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(s) => {
                let n: f64 = s.parse().unwrap();
                if n.fract() == 0.0 {
                    write!(f, "{}", n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
