use super::Expr;
use crate::environment::Env;

impl Expr {
    pub fn arity(&self, env: &Env) -> Option<usize> {
        match self {
            Expr::Lambda { .. } => Some(1),
            Expr::Variable(id) => env.arity(id),
            _ => None,
        }
    }

    /// TODO: `Option<T>` ではなく `Result<T, E>` を返すのが適切かも
    pub fn apply(&self, env: &Env, args: Vec<Expr>) -> Option<Expr> {
        match self {
            Expr::Lambda { param, body } => Some(body.clone().substitute(&param, &args[0])),

            Expr::Variable(id) => match env.get(&id) {
                Some(func) => Some(func.apply(args)),
                None => None,
            },

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function::Func;

    #[test]
    fn test_arity() {
        let f0 = Func::new("F0".into(), vec![], ":a".into());
        let f1 = Func::new("F1".into(), vec!["x".into()], ":a".into());
        let f2 = Func::new("F2".into(), vec!["x".into(), "y".into()], ":a".into());
        let f3 = Func::new(
            "F3".into(),
            vec!["x".into(), "y".into(), "z".into()],
            ":a".into(),
        );

        let env = Env::from(vec![f0, f1, f2, f3]);

        // シンボルは関数が紐づくことがない、arity は定義されない
        assert_eq!(Expr::s("a").arity(&env), None);

        // 関数適用の arity は定義されない
        assert_eq!(Expr::a(Expr::v("x"), Expr::v("y")).arity(&env), None);

        // ラムダ抽象の arity は常に 1
        assert_eq!(Expr::l("x".into(), Expr::v("x")).arity(&env), Some(1));

        // 関数として定義されていない自由変数の arity は定義されない (0ですらない)
        assert_eq!(Expr::v("x").arity(&env), None);

        // 定義済み関数と紐づく自由変数はその関数の arity を返す
        assert_eq!(Expr::v("F0").arity(&env), Some(0));
        assert_eq!(Expr::v("F1").arity(&env), Some(1));
        assert_eq!(Expr::v("F2").arity(&env), Some(2));
        assert_eq!(Expr::v("F3").arity(&env), Some(3));
    }

    #[test]
    fn test_apply() {
        // TODO
    }
}
