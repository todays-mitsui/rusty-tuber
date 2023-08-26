use crate::command::Command;
use crate::expression::display::ecmascript::ECMAScriptStyle as ExprECMAScriptStyle;
use crate::function::display::ecmascript::ECMAScriptStyle as FuncECMAScriptStyle;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct ECMAScriptStyle<'a>(pub &'a Command);

impl Display for ECMAScriptStyle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(func) => write!(f, "{}", FuncECMAScriptStyle(&func)),
            Command::Eval(e) => write!(f, "{}", ExprECMAScriptStyle(&e)),
            Command::EvalLast(e) => write!(f, "! {}", ExprECMAScriptStyle(&e)),
            Command::EvalHead(len, e) => write!(f, "!{} {}", len, ExprECMAScriptStyle(&e)),
            Command::EvalTail(len, e) => write!(f, "!-{} {}", len, ExprECMAScriptStyle(&e)),
            Command::Info(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(e) => write!(f, "?? {}", ExprECMAScriptStyle(&e)),
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
        assert_eq!(
            ECMAScriptStyle(&Command::Del("i".into())).to_string(),
            "i = i"
        );
    }

    #[test]
    fn test_update() {
        assert_eq!(
            ECMAScriptStyle(&Command::Update(Func::new(
                "i".into(),
                vec!["x".into()],
                "x".into()
            )))
            .to_string(),
            "i(x) = x"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::Update(Func::new(
                "k".into(),
                vec!["x".into(), "y".into()],
                "x".into()
            )))
            .to_string(),
            "k(x, y) = x"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::Update(Func::new(
                "s".into(),
                vec!["x".into(), "y".into(), "z".into()],
                Expr::a(
                    Expr::a("x".into(), "z".into()),
                    Expr::a("y".into(), "z".into())
                )
            )))
            .to_string(),
            "s(x, y, z) = x(z, y(z))"
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(ECMAScriptStyle(&Command::Eval("a".into())).to_string(), "a");

        assert_eq!(
            ECMAScriptStyle(&Command::Eval(Expr::a("a".into(), "b".into()))).to_string(),
            "a(b)"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::Eval(Expr::l("x".into(), "y".into()))).to_string(),
            "x => y"
        );
    }

    #[test]
    fn test_eval_last() {
        assert_eq!(
            ECMAScriptStyle(&Command::EvalLast("a".into())).to_string(),
            "! a"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::EvalLast(Expr::a("a".into(), "b".into()))).to_string(),
            "! a(b)"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::EvalLast(Expr::l("x".into(), "y".into()))).to_string(),
            "! x => y"
        );
    }

    #[test]
    fn test_eval_head() {
        assert_eq!(
            ECMAScriptStyle(&Command::EvalHead(42, "a".into())).to_string(),
            "!42 a"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::EvalHead(42, Expr::a("a".into(), "b".into()))).to_string(),
            "!42 a(b)"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::EvalHead(42, Expr::l("x".into(), "y".into()))).to_string(),
            "!42 x => y"
        );
    }

    #[test]
    fn test_eval_tail() {
        assert_eq!(
            ECMAScriptStyle(&Command::EvalTail(42, "a".into())).to_string(),
            "!-42 a"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::EvalTail(42, Expr::a("a".into(), "b".into()))).to_string(),
            "!-42 a(b)"
        );

        assert_eq!(
            ECMAScriptStyle(&Command::EvalTail(42, Expr::l("x".into(), "y".into()))).to_string(),
            "!-42 x => y"
        );
    }

    #[test]
    fn test_info() {
        assert_eq!(
            ECMAScriptStyle(&Command::Info("i".into())).to_string(),
            "? i"
        );
    }

    #[test]
    fn test_global() {
        assert_eq!(ECMAScriptStyle(&Command::Global).to_string(), "?");
    }
}
