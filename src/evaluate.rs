use crate::environment::Env;
use crate::expression::Expr;
use crate::expression::Expr::*;

pub struct EvalSteps<'a> {
    expr: Expr,
    stack: Stack,
    env: &'a Env,
}

impl EvalSteps<'_> {
    pub fn new(expr: Expr, env: &Env) -> EvalSteps {
        EvalSteps {
            expr,
            stack: Stack::new(),
            env,
        }
    }

    pub fn assemble(&self) -> Expr {
        let mut expr = self.expr.clone();

        for arg in self.stack.all() {
            expr = Expr::a(expr, arg);
        }

        expr
    }
}

impl Iterator for EvalSteps<'_> {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        while let Apply { lhs, rhs } = self.expr.clone() {
            self.expr = *lhs;
            self.stack.push(*rhs);
        }

        self.expr
            .arity(&self.env)
            .and_then(|a| self.stack.pop(a))
            .and_then(|args| self.expr.apply(&self.env, args))
            .map(|expr| {
                self.expr = expr;
                self.assemble()
            })
    }
}

// ========================================================================== //

#[derive(Debug)]
struct Stack(Vec<Expr>);

impl Stack {
    fn new() -> Stack {
        Stack(Vec::new())
    }

    fn push(&mut self, expr: Expr) {
        self.0.push(expr);
    }

    fn pop(&mut self, n: usize) -> Option<Vec<Expr>> {
        let length = self.len();

        if length >= n {
            Some(self.0.drain(length - n..).rev().collect())
        } else {
            None
        }
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
    use crate::function::Func;

    fn setup() -> Env {
        let i = Func::new(vec!["x".into()], "x".into());
        let k = Func::new(vec!["x".into(), "y".into()], "x".into());
        let s = Func::new(
            vec!["x".into(), "y".into(), "z".into()],
            Expr::a(
                Expr::a("x".into(), "z".into()),
                Expr::a("y".into(), "z".into()),
            ),
        );

        Env::from(vec![("i".into(), i), ("k".into(), k), ("s".into(), s)])
    }

    #[test]
    fn test_eval_steps_lambda_i() {
        let env = Env::new();

        let i = Expr::l("x".into(), "x".into());
        let expr = Expr::a(i, ":a".into());

        let mut steps = EvalSteps::new(expr, &env);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_1() {
        let env = Env::new();

        let k = Expr::l("x".into(), Expr::l("y".into(), "x".into()));
        let expr = Expr::a(k, ":a".into());

        let mut steps = EvalSteps::new(expr, &env);

        assert_eq!(steps.next(), Some(Expr::l("y".into(), ":a".into())));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_2() {
        let env = Env::new();

        let k = Expr::l("x".into(), Expr::l("y".into(), "x".into()));
        let expr = Expr::a(Expr::a(k, ":a".into()), ":b".into());

        let mut steps = EvalSteps::new(expr, &env);

        assert_eq!(
            steps.next(),
            Some(Expr::a(Expr::l("y".into(), ":a".into()), ":b".into()))
        );
        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_i() {
        let env = setup();

        let expr = Expr::a("i".into(), ":a".into());

        let mut steps = EvalSteps::new(expr, &env);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_k_1() {
        let env = setup();

        let expr = Expr::a("k".into(), ":a".into());

        let mut steps = EvalSteps::new(expr, &env);

        // k の arity が2なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_k_2() {
        let env = setup();

        let expr = Expr::a(Expr::a("k".into(), ":a".into()), ":b".into());

        let mut steps = EvalSteps::new(expr, &env);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_1() {
        let env = setup();

        let expr = Expr::a("s".into(), ":a".into());

        let mut steps = EvalSteps::new(expr, &env);

        // s の arity が3なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_2() {
        let env = setup();

        let expr = Expr::a(Expr::a("s".into(), ":a".into()), ":b".into());

        let mut steps = EvalSteps::new(expr, &env);

        // s の arity が3なのに対して引数を2つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_3() {
        let env = setup();

        let expr = Expr::a(
            Expr::a(Expr::a("s".into(), ":a".into()), ":b".into()),
            ":c".into(),
        );

        let mut steps = EvalSteps::new(expr, &env);

        assert_eq!(
            steps.next(),
            Some(Expr::a(
                Expr::a(":a".into(), ":c".into()),
                Expr::a(":b".into(), ":c".into())
            ))
        );
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = Stack(vec![Expr::v("x"), Expr::v("y")]);

        assert_eq!(stack.len(), 2);

        stack.push(Expr::v("z"));

        assert_eq!(stack.len(), 3);

        assert_eq!(stack.pop(2), Some(vec!["z".into(), "y".into()]));

        assert_eq!(stack.len(), 1);

        assert_eq!(stack.pop(1), Some(vec!["x".into()]));

        assert_eq!(stack.len(), 0);

        assert_eq!(stack.pop(1), None);
    }

    #[test]
    fn test_stack_all() {
        let stack = Stack(vec![Expr::v("x"), Expr::v("y"), Expr::v("z")]);
        assert_eq!(stack.all(), vec![Expr::v("z"), Expr::v("y"), Expr::v("x")]);

        let stack0 = Stack(vec![]);
        assert_eq!(stack0.all(), vec![]);
    }
}
