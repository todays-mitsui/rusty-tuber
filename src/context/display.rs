use crate::context::Context;
use std::fmt::Display;

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut context = self.0.iter().collect::<Vec<_>>();

        context.sort_by(|l, r| {
            let (l_name, r_name) = (l.0.as_ref(), r.0.as_ref());
            l_name.cmp(r_name)
        });

        write!(
            f,
            "{}",
            context
                .iter()
                .map(|(_, func)| func.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;
    use crate::function::Func;

    #[test]
    fn test_display() {
        let mut context = Context::new();
        context.def(Func::new(
            "i".into(),
            vec!["x".into()],
            Expr::Variable("x".into()),
        ));
        context.def(Func::new(
            "l".into(),
            vec!["x".into(), "y".into()],
            Expr::Variable("x".into()),
        ));
        context.def(Func::new(
            "K".into(),
            vec!["x".into(), "y".into()],
            Expr::Variable("x".into()),
        ));
        context.def(Func::new(
            "k".into(),
            vec!["x".into(), "y".into()],
            Expr::Variable("x".into()),
        ));

        assert_eq!(
            format!("{}", context),
            "
                ``Kxy = x\n\
                `ix = x\n\
                ``kxy = x\n\
                ``lxy = x
            "
            .trim()
        );
    }
}
