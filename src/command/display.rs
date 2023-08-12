use std::fmt::Display;

use crate::command::Command;
use crate::expression::Expr;
use crate::function::Func;
use crate::identifier::Ident;

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Del(i) => write!(f, "{} = {}", i, i),
            Command::Update(i, func) => write!(f, "{}", to_string(i, func)),
            Command::Eval(e) => write!(f, "{}", e),
            Command::Info(i) => write!(f, "? {}", i),
            Command::Global => write!(f, "?"),
            Command::Unlambda(e) => write!(f, "?? {}", e),
        }
    }
}

fn to_string(i: &Ident, f: &Func) -> String {
    let mut lhs = Expr::Variable(i.clone());
    for i in f.params() {
        lhs = Expr::a(lhs, Expr::Variable(i.clone()));
    }
    format!("{} = {}", lhs, f.body())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;

    #[test]
    fn test_del() {
        assert_eq!(Command::Del("i".into()).to_string(), "i = i");
    }

    #[test]
    fn test_update() {
        assert_eq!(
            Command::Update("i".into(), Func::new(vec!["x".into()], "x".into())).to_string(),
            "`ix = x"
        );

        assert_eq!(
            Command::Update(
                "k".into(),
                Func::new(vec!["x".into(), "y".into()], "x".into())
            )
            .to_string(),
            "``kxy = x"
        );

        assert_eq!(
            Command::Update(
                "s".into(),
                Func::new(
                    vec!["x".into(), "y".into(), "z".into()],
                    Expr::a(
                        Expr::a("x".into(), "z".into()),
                        Expr::a("y".into(), "z".into())
                    )
                )
            )
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
    fn test_info() {
        assert_eq!(Command::Info("i".into()).to_string(), "? i");
    }

    #[test]
    fn test_global() {
        assert_eq!(Command::Global.to_string(), "?");
    }
}
