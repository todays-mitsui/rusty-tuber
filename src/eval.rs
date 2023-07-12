use crate::env::Env;
use crate::expr::Expr;

pub struct Eval<'a> {
    expr: Expr,
    stack: Stack,
    env: &'a Env,
}

impl Eval<'_> {
    pub fn new(expr: Expr, env: &Env) -> Eval {
        let stack = Stack(vec![expr.clone()]);
        Eval { expr, stack, env }
    }

    // pub fn steps(&mut self) -> Vec<Expr> {
    //     let mut generator = || {
    //         while self.expr.is_apply() {
    //             let (lhs, rhs) = self.expr.destruct_apply();
    //             self.expr = lhs;
    //             self.stack.push(rhs);

    //             let maybe_arity = self.expr.arity(&self.env);

    //             if maybe_arity.map(|a| a <= self.stack.len()).unwrap_or(false) {
    //                 let arity = maybe_arity.unwrap();
    //                 let args = self.stack.pop(arity);
    //                 self.expr = self.expr.apply(&self.env, args);
    //             }
    //         }
    //     };
    // }
}

// ========================================================================== //

struct Stack(Vec<Expr>);

impl Stack {
    fn push(&mut self, expr: Expr) {
        self.0.push(expr);
    }

    fn pop(&mut self, n: usize) -> Vec<Expr> {
        let length = self.len();

        if length < n {
            panic!("stack underflow");
        }

        self.0.drain(length - n..).rev().collect()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[test]
fn test_stack() {
    let mut stack = Stack(vec![Expr::v("x"), Expr::v("y")]);

    assert_eq!(stack.len(), 2);

    stack.push(Expr::v("z"));

    assert_eq!(stack.len(), 3);

    assert_eq!(stack.pop(2), vec![Expr::v("z"), Expr::v("y")]);

    assert_eq!(stack.len(), 1);

    assert_eq!(stack.pop(1), vec![Expr::v("x")]);

    assert_eq!(stack.len(), 0);
}
