use expr::{Expr, Identifier};

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

    pub fn ality(&self) -> usize {
        self.params.len()
    }

    // copilot が生成した
    pub fn apply(&self, args: Vec<Expr>) -> Expr {
        let mut body = self.body.clone();
        for (param, arg) in self.params.iter().zip(args) {
            body = body.substitute(param, &arg);
        }
        body
    }
}
