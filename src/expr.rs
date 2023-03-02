pub mod parser;

#[derive(Debug, PartialEq)]
pub struct Identifier(String);

#[derive(Debug, PartialEq)]
pub enum Expr {
    Var(Identifier),
    Symbol(Identifier),
    Apply { lhs: Box<Expr>, rhs: Box<Expr> },
    Lambda { param: Identifier, body: Box<Expr> },
}

impl Expr {
    pub fn var(label: &str) -> Expr {
        Expr::Var(Identifier(String::from(label)))
    }

    pub fn symbol(label: &str) -> Expr {
        Expr::Symbol(Identifier(String::from(label)))
    }

    pub fn apply(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Apply{ lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
}
