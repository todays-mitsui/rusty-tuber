use crate::context::Context;
use crate::expression::Expr;
use crate::expression::Expr::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EvalSteps<'a> {
    expr: Expr,
    stack: Stack<'a>,
    context: &'a Context,
    step: Step,
}

/// 簡約のステップ
/// 最左最外簡約を行うために LeftTree → RightTree の順に簡約を試みる
/// 式全体を簡約し終えて正規形を得たら Done となる、それ以上簡約するべきものは何も無い
#[derive(Debug, Clone, PartialEq)]
enum Step {
    LeftTree,
    RightTree(usize),
    Done,
}

impl EvalSteps<'_> {
    pub fn new(expr: Expr, context: &Context) -> EvalSteps {
        EvalSteps {
            expr,
            stack: Stack::new(),
            context,
            step: Step::LeftTree,
        }
    }

    pub fn eval_last(&mut self, limit: usize) -> (Option<Expr>, bool) {
        assert!(0 < limit);

        if let Some(mut e) = self.next() {
            for _ in 0..limit - 1 {
                if let Some(next) = self.next() {
                    e = next;
                } else {
                    return (Some(e), false);
                }
            }

            // TODO: ここの true は嘘をつくことがある、peekable で先読みして正しい結果を返すように変える
            (Some(e), true)
        } else {
            (None, false)
        }
    }

    fn expr(&self) -> Expr {
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
        match self.step {
            Step::LeftTree => self.left_tree(),
            Step::RightTree(n) => self.right_tree(n),
            Step::Done => None,
        }
    }
}

impl EvalSteps<'_> {
    fn left_tree(&mut self) -> Option<Expr> {
        while let Apply { lhs, rhs } = self.expr.clone() {
            self.expr = *lhs;
            self.stack.push(EvalSteps::new(*rhs, self.context));
        }

        let maybe_next = self
            .expr
            .arity(&self.context)
            .filter(|a| *a >= 1 || self.stack.len() >= 1)
            .and_then(|a| self.stack.pop(a))
            .and_then(|args| {
                self.expr
                    .apply(&self.context, args.iter().map(|arg| arg.expr()).collect())
            })
            .map(|expr| {
                self.expr = expr;
                self.expr()
            });

        match maybe_next {
            Some(expr) => Some(expr),

            None => {
                self.step = Step::RightTree(0);
                self.next()
            }
        }
    }

    fn right_tree(&mut self, n: usize) -> Option<Expr> {
        match self.stack.nth(n) {
            // スタックの n 番目の枝を取得し、その枝の簡約を試みる
            Some(step) => match step.next() {
                Some(_) => Some(self.expr()),

                // n 番目の枝が簡約済みなら、n+1 番目の枝へ進む
                None => {
                    self.step = Step::RightTree(n + 1);
                    self.next()
                }
            },

            // n がスタックの長さを超えているなら、もう簡約するべきものは何も無い
            None => {
                self.step = Step::Done;
                self.next()
            }
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

    /// 末尾から数えて n 番目の要素を取得する
    fn nth(&mut self, n: usize) -> Option<&mut EvalSteps<'a>> {
        let len = self.0.len();
        if n >= len {
            None
        } else {
            self.0.get_mut(len - n - 1)
        }
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function::Func;

    fn setup() -> Context {
        let i = Func::new("i".into(), vec!["x".into()], "x".into());
        let k = Func::new("k".into(), vec!["x".into(), "y".into()], "x".into());
        let s = Func::new(
            "s".into(),
            vec!["x".into(), "y".into(), "z".into()],
            Expr::a(
                Expr::a("x".into(), "z".into()),
                Expr::a("y".into(), "z".into()),
            ),
        );

        let _true = Func::new("TRUE".into(), vec![], Expr::a("k".into(), "i".into()));
        let _false = Func::new("FALSE".into(), vec![], "k".into());

        Context::from(vec![i, k, s, _true, _false])
    }

    #[test]
    fn test_eval_steps_lambda_i() {
        let context = Context::new();

        let i = Expr::l("x".into(), "x".into());
        let expr = Expr::a(i, ":a".into());

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_1() {
        let context = Context::new();

        let k = Expr::l("x".into(), Expr::l("y".into(), "x".into()));
        let expr = Expr::a(k, ":a".into());

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(Expr::l("y".into(), ":a".into())));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_lambda_k_2() {
        let context = Context::new();

        let k = Expr::l("x".into(), Expr::l("y".into(), "x".into()));
        let expr = Expr::a(Expr::a(k, ":a".into()), ":b".into());

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next(),
            Some(Expr::a(Expr::l("y".into(), ":a".into()), ":b".into()))
        );
        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_true_1() {
        let context = setup();

        let expr: Expr = "TRUE".into();

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_true_2() {
        let context = setup();

        let expr = Expr::a(":a".into(), "TRUE".into());

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_true_3() {
        let context = setup();

        let expr = Expr::a(Expr::a("TRUE".into(), ":a".into()), ":b".into());

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next(),
            Some(Expr::a(
                Expr::a(Expr::a("k".into(), "i".into()), ":a".into()),
                ":b".into()
            ))
        );
        assert_eq!(steps.next(), Some(Expr::a("i".into(), ":b".into())));
        assert_eq!(steps.next(), Some(":b".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_i() {
        let context = setup();

        let expr = Expr::a("i".into(), ":a".into());

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_k_1() {
        let context = setup();

        let expr = Expr::a("k".into(), ":a".into());

        let mut steps = EvalSteps::new(expr, &context);

        // k の arity が2なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_k_2() {
        let context = setup();

        let expr = Expr::a(Expr::a("k".into(), ":a".into()), ":b".into());

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(":a".into()));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_1() {
        let context = setup();

        let expr = Expr::a("s".into(), ":a".into());

        let mut steps = EvalSteps::new(expr, &context);

        // s の arity が3なのに対して引数を1つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_2() {
        let context = setup();

        let expr = Expr::a(Expr::a("s".into(), ":a".into()), ":b".into());

        let mut steps = EvalSteps::new(expr, &context);

        // s の arity が3なのに対して引数を2つしか与えていないので簡約されない
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_func_s_3() {
        let context = setup();

        let expr = Expr::a(
            Expr::a(Expr::a("s".into(), ":a".into()), ":b".into()),
            ":c".into(),
        );

        let mut steps = EvalSteps::new(expr, &context);

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
    fn test_eval_steps_skk() {
        let context = setup();

        let expr = Expr::a(
            Expr::a(Expr::a("s".into(), "k".into()), "k".into()),
            ":a".into(),
        );

        let steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.last(), Some(Expr::s("a")));
    }

    #[test]
    fn test_eval_steps_right_tree_1() {
        let context = setup();

        let expr = Expr::a(
            ":a".into(),
            Expr::a(Expr::a("k".into(), ":b".into()), ":c".into()),
        );

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.next(), Some(Expr::a(":a".into(), ":b".into())));
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps_right_tree_2() {
        let context = setup();

        // ```:a`i:b`i:c
        let expr = Expr::a(
            Expr::a(":a".into(), Expr::a("i".into(), ":b".into())),
            Expr::a("i".into(), ":c".into()),
        );

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next(),
            Some(Expr::a(
                Expr::a(":a".into(), ":b".into(),),
                Expr::a("i".into(), ":c".into())
            ))
        );
        assert_eq!(
            steps.next(),
            Some(Expr::a(Expr::a(":a".into(), ":b".into(),), ":c".into()))
        );
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_eval_steps() {
        let context = setup();

        // ```s^x.`x:a^x.`x:b:c
        let expr = Expr::a(
            Expr::a(
                Expr::a(
                    "s".into(),
                    Expr::l("x".into(), Expr::a("x".into(), ":a".into())),
                ),
                Expr::l("x".into(), Expr::a("x".into(), ":b".into())),
            ),
            ":c".into(),
        );

        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.next(),
            // ``^x.`x:a:c`^x.`x:b:c
            Some(Expr::a(
                Expr::a(
                    Expr::l("x".into(), Expr::a("x".into(), ":a".into())),
                    ":c".into()
                ),
                Expr::a(
                    Expr::l("x".into(), Expr::a("x".into(), ":b".into())),
                    ":c".into()
                )
            ))
        );
        assert_eq!(
            steps.next(),
            // ``:c:a`^x.`x:b:c
            Some(Expr::a(
                Expr::a(":c".into(), ":a".into()),
                Expr::a(
                    Expr::l("x".into(), Expr::a("x".into(), ":b".into())),
                    ":c".into()
                )
            ))
        );
        assert_eq!(
            steps.next(),
            // ``:c:a`:c:b
            Some(Expr::a(
                Expr::a(":c".into(), ":a".into()),
                Expr::a(":c".into(), ":b".into()),
            ))
        );
        assert_eq!(steps.next(), None);
    }

    #[test]
    fn test_stack_pop() {
        let context = Context::new();
        let mut stack = Stack(vec![
            EvalSteps::new(Expr::v("x"), &context),
            EvalSteps::new(Expr::v("y"), &context),
        ]);

        assert_eq!(stack.len(), 2);

        stack.push(EvalSteps::new(Expr::v("z"), &context));

        assert_eq!(stack.len(), 3);

        assert_eq!(
            stack.pop(2),
            Some(vec![
                EvalSteps::new(Expr::v("z"), &context),
                EvalSteps::new(Expr::v("y"), &context)
            ])
        );

        assert_eq!(stack.len(), 1);

        assert_eq!(
            stack.pop(1),
            Some(vec![EvalSteps::new(Expr::v("x"), &context)])
        );

        assert_eq!(stack.len(), 0);

        assert_eq!(stack.pop(1), None);
    }

    #[test]
    fn test_stack_all() {
        let context = Context::new();
        let stack = Stack(vec![
            EvalSteps::new(Expr::v("x"), &context),
            EvalSteps::new(Expr::v("y"), &context),
            EvalSteps::new(Expr::v("z"), &context),
        ]);
        assert_eq!(
            stack.all(),
            vec![
                EvalSteps::new(Expr::v("z"), &context),
                EvalSteps::new(Expr::v("y"), &context),
                EvalSteps::new(Expr::v("x"), &context),
            ]
        );

        let stack0 = Stack(vec![]);
        assert_eq!(stack0.all(), vec![]);
    }

    #[test]
    fn test_stack_nth() {
        let context = Context::new();
        let mut stack = Stack(vec![
            EvalSteps::new(Expr::v("x"), &context),
            EvalSteps::new(Expr::v("y"), &context),
            EvalSteps::new(Expr::v("z"), &context),
        ]);

        assert_eq!(
            stack.nth(0),
            Some(&mut EvalSteps::new(Expr::v("z"), &context))
        );
        assert_eq!(
            stack.nth(1),
            Some(&mut EvalSteps::new(Expr::v("y"), &context))
        );
        assert_eq!(
            stack.nth(2),
            Some(&mut EvalSteps::new(Expr::v("x"), &context))
        );
        assert_eq!(stack.nth(3), None);
    }

    #[test]
    fn test_eval_last_1() {
        let context = setup();

        let expr = ":a".into();
        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.eval_last(42), (None, false));
    }

    #[test]
    fn test_eval_last_2() {
        let context = setup();

        let expr = Expr::a(
            "i".into(),
            Expr::a(
                "i".into(),
                Expr::a("i".into(), Expr::a("i".into(), ":a".into())),
            ),
        );
        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(steps.eval_last(42), (Some(":a".into()), false));
    }

    #[test]
    fn test_eval_last_3() {
        let context = setup();

        let expr = Expr::a(
            "i".into(),
            Expr::a(
                "i".into(),
                Expr::a("i".into(), Expr::a("i".into(), ":a".into())),
            ),
        );
        let mut steps = EvalSteps::new(expr, &context);

        assert_eq!(
            steps.eval_last(3),
            (Some(Expr::a("i".into(), ":a".into())), true)
        );
    }
}
