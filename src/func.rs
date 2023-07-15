use crate::expr::{Expr, Identifier};

#[derive(Debug, PartialEq)]
pub struct Func {
    params: Vec<Identifier>,
    body: Expr,
}

impl Func {
    pub fn new(params: Vec<Identifier>, body: Expr) -> Func {
        Func { params, body }
    }

    // copilot が勝手に生成した、怖っ
    // pub fn apply(&self, args: Vec<Expr>) -> Expr {
    //     let mut body = self.body.clone();
    //     for (param, arg) in self.params.iter().zip(args) {
    //         body = body.substitute(param, &arg);
    //     }
    //     body
    // }

    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn apply(&self, args: Vec<Expr>) -> Expr {
        let mut body = self.body.clone();
        for (param, arg) in self.params.iter().zip(args) {
            body = body.substitute(param, &arg);
        }
        body
    }
}

pub fn i() -> Func {
    Func {
        params: vec![Identifier::new("x")],
        body: Expr::v("x"),
    }
}

pub fn k() -> Func {
    Func {
        params: vec![Identifier::new("x"), Identifier::new("y")],
        body: Expr::v("x"),
    }
}

pub fn s() -> Func {
    Func {
        params: vec![
            Identifier::new("x"),
            Identifier::new("y"),
            Identifier::new("z"),
        ],
        body: Expr::a(
            Expr::a(Expr::v("x"), Expr::v("z")),
            Expr::a(Expr::v("y"), Expr::v("z")),
        ),
    }
}
