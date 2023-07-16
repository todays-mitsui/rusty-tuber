mod free_vars;
pub mod parser;
mod substitute;

use crate::env::Env;
use crate::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Variable(Identifier),
    Symbol(Identifier),
    Apply { lhs: Box<Expr>, rhs: Box<Expr> },
    Lambda { param: Identifier, body: Box<Expr> },
}

impl Expr {
    pub fn destruct_apply(self) -> (Expr, Expr) {
        match self {
            Expr::Apply { lhs, rhs } => (*lhs, *rhs),
            _ => panic!("destruct_apply: not an apply"),
        }
    }

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

    pub fn arity(&self, env: &Env) -> Option<usize> {
        match self {
            Expr::Lambda { .. } => Some(1),
            Expr::Variable(id) => env.arity(id),
            _ => None,
        }
    }
}

impl Expr {
    pub fn apply(&self, env: &Env, args: Vec<Expr>) -> Expr {
        match self {
            Expr::Lambda { param, body } => body.clone().substitute(&param, &args[0]),

            Expr::Variable(id) => match env.get(&id) {
                Some(func) => func.apply(args),
                None => panic!("apply: not found"),
            },

            _ => panic!("apply: not a function"),
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
