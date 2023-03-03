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
}
