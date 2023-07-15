use crate::env::Env;
use crate::expr::Expr;
use crate::expr::Expr::*;

pub struct EvalSteps {
    expr: Expr,
    stack: Stack,
    env: Env,
}

impl EvalSteps {
    pub fn new(expr: Expr, env: Env) -> EvalSteps {
        let stack = Stack(vec![expr.clone()]);
        EvalSteps { expr, stack, env }
    }

    pub fn assemble(&self) -> Expr {
        let mut expr = self.expr.clone();

        for arg in self.stack.all() {
            expr = Expr::a(expr, arg);
        }

        expr
    }
}

impl Iterator for EvalSteps {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        while let Apply { lhs, rhs } = self.expr.clone() {
            self.expr = *lhs;
            self.stack.push(*rhs);

            let maybe_arity = self.expr.arity(&self.env);

            if maybe_arity.map(|a| a <= self.stack.len()).unwrap_or(false) {
                let arity = maybe_arity.unwrap();
                let mut args = self.stack.pop(arity);
                args.reverse();

                self.expr = self.expr.apply(&self.env, args);

                return Some(self.assemble());
            }
        }

        return None;
    }
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

    fn all(&self) -> Vec<Expr> {
        let mut all = self.0.clone();
        all.reverse();
        all
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::func::Func;

    #[test]
    fn test_eval_steps_func() {
        let i = Func::new(vec!["x".into()], "x".into());
        let k = Func::new(vec!["x".into(), "y".into()], "x".into());
        let s = Func::new(
            vec!["x".into(), "y".into(), "z".into()],
            Expr::a(
                Expr::a("x".into(), "z".into()),
                Expr::a("y".into(), "z".into()),
            ),
        );

        let env = Env::from(vec![("i".into(), i), ("k".into(), k), ("s".into(), s)]);

        let expr = Expr::a("i".into(), ":a".into());

        let mut steps = EvalSteps::new(expr, env);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = Stack(vec![Expr::v("x"), Expr::v("y")]);

        assert_eq!(stack.len(), 2);

        stack.push(Expr::v("z"));

        assert_eq!(stack.len(), 3);

        assert_eq!(stack.pop(2), vec![Expr::v("z"), Expr::v("y")]);

        assert_eq!(stack.len(), 1);

        assert_eq!(stack.pop(1), vec![Expr::v("x")]);

        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_all() {
        let stack = Stack(vec![Expr::v("x"), Expr::v("y"), Expr::v("z")]);

        assert_eq!(stack.all(), vec![Expr::v("z"), Expr::v("y"), Expr::v("x")]);
    }
}
