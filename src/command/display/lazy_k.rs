use crate::command::Command;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct LazyKStyle<'a>(pub &'a Command);

impl Display for LazyKStyle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;
    use crate::function::Func;

    #[test]
    fn test_del() {
        assert_eq!(LazyKStyle(&Command::Del("i".into())).to_string(), "i = i");
    }

    #[test]
    fn test_update() {
        assert_eq!(
            LazyKStyle(&Command::Update(Func::new(
                "i".into(),
                vec!["x".into()],
                "x".into()
            )))
            .to_string(),
            "`ix = x"
        );

        assert_eq!(
            LazyKStyle(&Command::Update(Func::new(
                "k".into(),
                vec!["x".into(), "y".into()],
                "x".into()
            )))
            .to_string(),
            "``kxy = x"
        );

        assert_eq!(
            LazyKStyle(&Command::Update(Func::new(
                "s".into(),
                vec!["x".into(), "y".into(), "z".into()],
                Expr::a(
                    Expr::a("x".into(), "z".into()),
                    Expr::a("y".into(), "z".into())
                )
            )))
            .to_string(),
            "```sxyz = ``xz`yz"
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(LazyKStyle(&Command::Eval("a".into())).to_string(), "a");

        assert_eq!(
            LazyKStyle(&Command::Eval(Expr::a("a".into(), "b".into()))).to_string(),
            "`ab"
        );

        assert_eq!(
            LazyKStyle(&Command::Eval(Expr::l("x".into(), "y".into()))).to_string(),
            "^x.y"
        );
    }

    #[test]
    fn test_eval_last() {
        assert_eq!(
            LazyKStyle(&Command::EvalLast("a".into())).to_string(),
            "! a"
        );

        assert_eq!(
            LazyKStyle(&Command::EvalLast(Expr::a("a".into(), "b".into()))).to_string(),
            "! `ab"
        );

        assert_eq!(
            LazyKStyle(&Command::EvalLast(Expr::l("x".into(), "y".into()))).to_string(),
            "! ^x.y"
        );
    }

    #[test]
    fn test_eval_head() {
        assert_eq!(
            LazyKStyle(&Command::EvalHead(42, "a".into())).to_string(),
            "!42 a"
        );

        assert_eq!(
            LazyKStyle(&Command::EvalHead(42, Expr::a("a".into(), "b".into()))).to_string(),
            "!42 `ab"
        );

        assert_eq!(
            LazyKStyle(&Command::EvalHead(42, Expr::l("x".into(), "y".into()))).to_string(),
            "!42 ^x.y"
        );
    }

    #[test]
    fn test_eval_tail() {
        assert_eq!(
            LazyKStyle(&Command::EvalTail(42, "a".into())).to_string(),
            "!-42 a"
        );

        assert_eq!(
            LazyKStyle(&Command::EvalTail(42, Expr::a("a".into(), "b".into()))).to_string(),
            "!-42 `ab"
        );

        assert_eq!(
            LazyKStyle(&Command::EvalTail(42, Expr::l("x".into(), "y".into()))).to_string(),
            "!-42 ^x.y"
        );
    }

    #[test]
    fn test_info() {
        assert_eq!(LazyKStyle(&Command::Info("i".into())).to_string(), "? i");
    }

    #[test]
    fn test_global() {
        assert_eq!(LazyKStyle(&Command::Global).to_string(), "?");
    }
}
