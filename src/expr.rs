mod apply;
mod free_vars;
pub mod parser;
mod substitute;

use crate::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// 変数
    Variable(Identifier),

    /// シンボル
    Symbol(Identifier),

    /// 適用
    Apply { lhs: Box<Expr>, rhs: Box<Expr> },

    /// ラムダ抽象
    Lambda { param: Identifier, body: Box<Expr> },
}

impl Expr {
    /// 変数を作る
    pub fn v(label: &str) -> Expr {
        Expr::Variable(Identifier::new(label))
    }

    /// シンボルを作る
    pub fn s(label: &str) -> Expr {
        Expr::Symbol(Identifier::new(label))
    }

    /// 適用を作る
    pub fn a(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Apply {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    /// ラムダ抽象を作る
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
