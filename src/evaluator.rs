use crate::{
    enviroment::Enviroment,
    literal::Literal,
    operator::Operator,
    parser::{expression::Expr, statement::Stmt},
};

pub struct Evaluator {
    pub enviroment: Enviroment,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            enviroment: Enviroment::new(None),
        }
    }

    pub fn scope_in(&mut self) {
        let parent = std::mem::replace(&mut self.enviroment, Enviroment::new(None));
        self.enviroment = Enviroment::new(Some(Box::new(parent)));
    }

    pub fn scope_out(&mut self) {
        let parent = self.enviroment.enviroment.take();
        if let Some(parent) = parent {
            self.enviroment = *parent;
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Literal {
        match expr {
            Expr::Literal(literal) => literal.clone(),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary { prefix, expr } => match prefix {
                Operator::Bang => match self.evaluate(expr) {
                    Literal::Boolean(false) => Literal::Boolean(true),
                    Literal::Nil => Literal::Boolean(true),
                    _ => Literal::Boolean(false),
                },
                Operator::Minus => match self.evaluate(expr) {
                    Literal::Number(n) => {
                        let n: f64 = n.parse().unwrap();
                        Literal::Number(format!("{}", -n))
                    }
                    _ => std::process::exit(70),
                },
                _ => todo!(),
            },
            Expr::Binary { op, first, second } => match op {
                Operator::Star => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Number(format!("{}", a * b))
                    }
                    _ => std::process::exit(70),
                },
                Operator::Slash => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Number(format!("{}", a / b))
                    }
                    _ => std::process::exit(70),
                },
                Operator::Plus => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Number(format!("{}", a + b))
                    }
                    (Literal::String(a), Literal::String(b)) => {
                        Literal::String(format!("{}{}", a, b))
                    }
                    _ => std::process::exit(70),
                },
                Operator::Minus => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Number(format!("{}", a - b))
                    }
                    _ => std::process::exit(70),
                },
                Operator::Greater => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Boolean(a > b)
                    }
                    _ => std::process::exit(70),
                },
                Operator::Less => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Boolean(a < b)
                    }
                    _ => std::process::exit(70),
                },
                Operator::GreaterEqual => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Boolean(a >= b)
                    }
                    _ => std::process::exit(70),
                },
                Operator::LessEqual => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Boolean(a <= b)
                    }
                    _ => std::process::exit(70),
                },
                Operator::EqualEqual => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Boolean(a == b)
                    }
                    (Literal::Number(a), Literal::String(b)) => Literal::Boolean(false),
                    (Literal::String(a), Literal::Number(b)) => Literal::Boolean(false),
                    (Literal::String(a), Literal::String(b)) => Literal::Boolean(a == b),
                    (Literal::Boolean(a), Literal::Boolean(b)) => Literal::Boolean(a == b),
                    _ => todo!(),
                },
                Operator::BangEqual => match (self.evaluate(first), self.evaluate(second)) {
                    (Literal::Number(a), Literal::Number(b)) => {
                        let a: f64 = a.parse().unwrap();
                        let b: f64 = b.parse().unwrap();
                        Literal::Boolean(a != b)
                    }
                    (Literal::String(a), Literal::String(b)) => Literal::Boolean(a != b),
                    (Literal::Boolean(a), Literal::Boolean(b)) => Literal::Boolean(a != b),
                    _ => todo!(),
                },
                _ => todo!(),
            },
            Expr::Identifier(identifier) => match self.get(identifier) {
                Some(literal) => literal.clone(),
                _ => std::process::exit(70),
            },
            Expr::Assign { name, value } => {
                let val = self.evaluate(value);
                if !self.enviroment.assign(name.clone(), val.clone()) {
                    eprintln!("Undefined variable '{}'.", name);
                    std::process::exit(70);
                }
                val
            }
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => println!("{}", self.evaluate(expr)),
            Stmt::Var { name, initializer } => {
                let val = match initializer {
                    Some(expr) => self.evaluate(expr),
                    None => Literal::Nil,
                };
                self.enviroment.declare(name.clone(), val);
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr);
            }
        }
    }

    pub fn get(&self, k: &String) -> Option<&Literal> {
        self.enviroment.get(k)
    }
}
