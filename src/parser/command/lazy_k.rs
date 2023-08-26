use crate::command::Command;
use crate::parser::command::parse_command as parent;

pub fn parse_command(s: &str) -> Result<Command, String> {
    parent(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::expression::Expr;
    use crate::function::Func;

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("f=g"),
            Ok(Command::Update(Func::new("f".into(), vec![], "g".into())))
        );

        assert_eq!(
            parse_command("`ix = x"),
            Ok(Command::Update(Func::new(
                "i".into(),
                vec!["x".into()],
                "x".into()
            )))
        );

        assert_eq!(
            parse_command("```sxyz = ``xz`yz"),
            Ok(Command::Update(Func::new(
                "s".into(),
                vec!["x".into(), "y".into(), "z".into()],
                Expr::a(
                    Expr::a("x".into(), "z".into()),
                    Expr::a("y".into(), "z".into())
                )
            )))
        );

        assert_eq!(
            parse_command("`ab"),
            Ok(Command::Eval(Expr::a("a".into(), "b".into())))
        );

        assert_eq!(parse_command("? a"), Ok(Command::Info("a".into())));

        assert_eq!(parse_command("?"), Ok(Command::Global));

        assert!(parse_command("f=g h=i").is_err());
    }
}
