mod apply;
mod free_vars;
pub mod parser;
mod substitute;

use crate::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Variable(Identifier),
    Symbol(Identifier),
    Apply { lhs: Box<Expr>, rhs: Box<Expr> },
    Lambda { param: Identifier, body: Box<Expr> },
}

impl Expr {
    pub fn v(label: &str) -> Expr {
        Expr::Variable(Identifier::new(label))
    }

    pub fn s(label: &str) -> Expr {
        Expr::Symbol(Identifier::new(label))
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
}

impl From<&str> for Expr {
    fn from(s: &str) -> Self {
        match s.chars().nth(0) {
            Some(':') => Expr::Symbol(Identifier::new(&s[1..])),
            Some(_) => Expr::Variable(Identifier::new(s)),
            _ => panic!("invalid identifier"),
        }
    }
}
