pub mod parser;

#[derive(Debug, PartialEq)]
pub struct Identifier(String);

#[derive(Debug, PartialEq)]
pub enum Expr {
    Variable(Identifier),
    Symbol(Identifier),
    Apply { lhs: Box<Expr>, rhs: Box<Expr> },
    Lambda { param: Identifier, body: Box<Expr> },
}

impl Expr {
    pub fn v(label: &str) -> Expr {
        Expr::Variable(Identifier(String::from(label)))
    }

    pub fn s(label: &str) -> Expr {
        Expr::Symbol(Identifier(String::from(label)))
    }

    pub fn a(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Apply {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn l(param: Identifier, body: Expr) -> Expr {
        Expr::Lambda {
            param,
            body: Box::new(body),
        }
    }

    fn substitute(self, param: &Identifier, arg: &Expr) -> Expr {
        match self {
            Expr::Variable(id) => {
                if &id == param {
                    *arg.clone()
                } else {
                    Expr::Variable(id)
                }
            },
            Expr::Symbol(_) => self.clone(),
            Expr::Apply { lhs, rhs } => Expr::Apply {
                lhs: Box::new(lhs.substitute(param, arg)),
                rhs: Box::new(rhs.substitute(param, arg)),
            },
            Expr::Lambda { param: p, body } => {
                if &p == param {
                    Expr::Lambda { param: p, body }
                } else {
                    Expr::Lambda {
                        param: p,
                        body: Box::new(body.substitute(param, arg)),
                    }
                }
            },
        }
    }
}

pub fn eval(lhs: Expr, rhs: Expr) -> Expr {
    match lhs {
        Expr::Lambda { param, body } => body.substitute(&param, &rhs),
        _ => panic!("not a lambda"),
    }
}
