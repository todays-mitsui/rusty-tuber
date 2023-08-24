use std::fmt::Display;

use crate::command::Command;

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(func) => write!(f, "{}", func),
            Command::Eval(e) => write!(f, "{}", e),
            Command::EvalLast(e) => write!(f, "! {}", e),
            Command::EvalHead(len, e) => write!(f, "!{} {}", len, e),
            Command::EvalTail(len, e) => write!(f, "!-{} {}", len, e),
            Command::Info(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(e) => write!(f, "?? {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;
    use crate::function::Func;

    #[test]
    fn test_del() {
        assert_eq!(Command::Del("i".into()).to_string(), "i = i");
    }

    #[test]
    fn test_update() {
        assert_eq!(
            Command::Update(Func::new("i".into(), vec!["x".into()], "x".into())).to_string(),
            "`ix = x"
        );

        assert_eq!(
            Command::Update(Func::new(
                "k".into(),
                vec!["x".into(), "y".into()],
                "x".into()
            ))
            .to_string(),
            "``kxy = x"
        );

        assert_eq!(
            Command::Update(Func::new(
                "s".into(),
                vec!["x".into(), "y".into(), "z".into()],
                Expr::a(
                    Expr::a("x".into(), "z".into()),
                    Expr::a("y".into(), "z".into())
                )
            ))
            .to_string(),
            "```sxyz = ``xz`yz"
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(Command::Eval("a".into()).to_string(), "a");

        assert_eq!(
            Command::Eval(Expr::a("a".into(), "b".into())).to_string(),
            "`ab"
        );

        assert_eq!(
            Command::Eval(Expr::l("x".into(), "y".into())).to_string(),
            "^x.y"
        );
    }

    #[test]
    fn test_eval_last() {
        assert_eq!(Command::EvalLast("a".into()).to_string(), "! a");

        assert_eq!(
            Command::EvalLast(Expr::a("a".into(), "b".into())).to_string(),
            "! `ab"
        );

        assert_eq!(
            Command::EvalLast(Expr::l("x".into(), "y".into())).to_string(),
            "! ^x.y"
        );
    }

    #[test]
    fn test_eval_head() {
        assert_eq!(Command::EvalHead(42, "a".into()).to_string(), "!42 a");

        assert_eq!(
            Command::EvalHead(42, Expr::a("a".into(), "b".into())).to_string(),
            "!42 `ab"
        );

        assert_eq!(
            Command::EvalHead(42, Expr::l("x".into(), "y".into())).to_string(),
            "!42 ^x.y"
        );
    }

    #[test]
    fn test_eval_tail() {
        assert_eq!(Command::EvalTail(42, "a".into()).to_string(), "!-42 a");

        assert_eq!(
            Command::EvalTail(42, Expr::a("a".into(), "b".into())).to_string(),
            "!-42 `ab"
        );

        assert_eq!(
            Command::EvalTail(42, Expr::l("x".into(), "y".into())).to_string(),
            "!-42 ^x.y"
        );
    }

    #[test]
    fn test_info() {
        assert_eq!(Command::Info("i".into()).to_string(), "? i");
    }

    #[test]
    fn test_global() {
        assert_eq!(Command::Global.to_string(), "?");
    }
}
