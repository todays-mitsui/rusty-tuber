use crate::environment::Env;
use crate::expression::Expr;
use crate::expression::Expr::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EvalSteps<'a> {
    expr: Expr,
    stack: Stack<'a>,
    env: &'a Env,
    status: Status,
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    LeftTree,
    RightTree(usize),
    Done,
}

impl EvalSteps<'_> {
    pub fn new(expr: Expr, env: &Env) -> EvalSteps {
        EvalSteps {
            expr,
            stack: Stack::new(),
            env,

            status: Status::LeftTree,
        }
    }

    fn expr(&self) -> Expr {
        let mut expr = self.expr.clone();

        for arg in self.stack.all() {
            expr = Expr::a(expr, arg.expr());
        }

        expr
    }

    fn assemble(&self) -> Expr {
        let mut expr = self.expr.clone();

        for arg in self.stack.all() {
            expr = Expr::a(expr, arg.expr());
        }

        expr
    }
}

impl Iterator for EvalSteps<'_> {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        match self.status {
            Status::LeftTree => self.left_tree(),
            Status::RightTree(n) => self.right_tree(n),
            Status::Done => None,
        }
    }
}

impl EvalSteps<'_> {
    fn left_tree(&mut self) -> Option<Expr> {
        while let Apply { lhs, rhs } = self.expr.clone() {
            self.expr = *lhs;
            self.stack.push(EvalSteps::new(*rhs, self.env));
        }

        let maybe_next = self
            .expr
            .arity(&self.env)
            .and_then(|a| self.stack.pop(a))
            .and_then(|args| {
                self.expr
                    .apply(&self.env, args.iter().map(|arg| arg.expr()).collect())
            })
            .map(|expr| {
                self.expr = expr;
                self.assemble()
            });

        match maybe_next {
            Some(expr) => Some(expr),

            None => {
                self.status = Status::RightTree(0);
                self.next()
            }
        }
    }

    fn right_tree(&mut self, n: usize) -> Option<Expr> {
        match self.stack.nth(n) {
            None => {
                self.status = Status::Done;
                self.next()
            }

            Some(step) => match step.next() {
                None => {
                    self.status = Status::RightTree(n + 1);
                    self.next()
                }

                Some(expr) => Some(expr),
            },
        }
    }
}

// ========================================================================== //

#[derive(Debug, Clone, PartialEq)]
struct Stack<'a>(Vec<EvalSteps<'a>>);

impl<'a> Stack<'a> {
    fn new() -> Stack<'a> {
        Stack(Vec::new())
    }

    fn push(&mut self, expr: EvalSteps<'a>) {
        self.0.push(expr);
    }

    fn pop(&mut self, n: usize) -> Option<Vec<EvalSteps>> {
        let length = self.len();

        if length >= n {
            Some(self.0.drain(length - n..).rev().collect())
        } else {
            None
        }
    }

    fn all(&self) -> Vec<EvalSteps> {
        let mut all = self.0.clone();
        all.reverse();
        all
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn nth(&mut self, n: usize) -> Option<&mut EvalSteps<'a>> {
        self.0.get_mut(n)
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
    fn test_eval_steps() {
        let env = setup();

        let expr = Expr::a(
            Expr::a(Expr::a("s".into(), "k".into()), "k".into()),
            ":a".into(),
        );

        let steps = EvalSteps::new(expr, &env);

        assert_eq!(steps.last(), Some(Expr::s("a")));
    }

    #[test]
    fn test_stack_pop() {
        let env = Env::new();
        let mut stack = Stack(vec![
            EvalSteps::new(Expr::v("x"), &env),
            EvalSteps::new(Expr::v("y"), &env),
        ]);

        assert_eq!(stack.len(), 2);

        stack.push(EvalSteps::new(Expr::v("z"), &env));

        assert_eq!(stack.len(), 3);

        assert_eq!(
            stack.pop(2),
            Some(vec![
                EvalSteps::new(Expr::v("z"), &env),
                EvalSteps::new(Expr::v("y"), &env)
            ])
        );

        assert_eq!(stack.len(), 1);

        assert_eq!(stack.pop(1), Some(vec![EvalSteps::new(Expr::v("x"), &env)]));

        assert_eq!(stack.len(), 0);

        assert_eq!(stack.pop(1), None);
    }

    #[test]
    fn test_stack_all() {
        let env = Env::new();
        let stack = Stack(vec![
            EvalSteps::new(Expr::v("x"), &env),
            EvalSteps::new(Expr::v("y"), &env),
            EvalSteps::new(Expr::v("z"), &env),
        ]);
        assert_eq!(
            stack.all(),
            vec![
                EvalSteps::new(Expr::v("z"), &env),
                EvalSteps::new(Expr::v("y"), &env),
                EvalSteps::new(Expr::v("x"), &env),
            ]
        );

        let stack0 = Stack(vec![]);
        assert_eq!(stack0.all(), vec![]);
    }
}
