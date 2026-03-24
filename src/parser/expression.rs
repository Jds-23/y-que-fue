use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(s) => {
                let n: f64 = s.parse().unwrap();
                let out = if n.fract() == 0.0 {
                    format!("{:.1}", n)
                } else {
                    format!("{}", n)
                };
                write!(f, "{}", out)
            }
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Grouping(group) => write!(f, "(group {})", group),
        }
    }
}
