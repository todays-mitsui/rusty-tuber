use crate::expression::display::ecmascript::ECMAScriptStyle as ExprECMAScriptStyle;
use crate::function::Func;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct ECMAScriptStyle<'a>(pub &'a Func);

impl Display for ECMAScriptStyle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.arity() == 0 {
            write!(
                f,
                "{} = {}",
                self.0.name.label(),
                ExprECMAScriptStyle(&self.0.body)
            )
        } else {
            write!(
                f,
                "{}({}) = {}",
                self.0.name.label(),
                self.0
                    .params
                    .iter()
                    .map(|i| i.label())
                    .collect::<Vec<_>>()
                    .join(", "),
                ExprECMAScriptStyle(&self.0.body)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;

    #[test]
    fn test_to_string_0() {
        let f = Func {
            name: "TRUE".into(),
            params: vec![],
            body: "k".into(),
        };
        assert_eq!(ECMAScriptStyle(&f).to_string(), "TRUE = k");
    }

    #[test]
    fn test_to_string_1() {
        let f = Func {
            name: "f".into(),
            params: vec!["x".into(), "y".into()],
            body: Expr::a("x".into(), "y".into()),
        };
        assert_eq!(ECMAScriptStyle(&f).to_string(), "f(x, y) = x(y)");
    }

    #[test]
    fn test_to_string_2() {
        let f = Func {
            name: "F".into(),
            params: vec!["X".into(), "Y".into()],
            body: Expr::a("X".into(), "Y".into()),
        };
        assert_eq!(ECMAScriptStyle(&f).to_string(), "F(X, Y) = X(Y)");
    }

    #[test]
    fn test_to_string_3() {
        let f = Func {
            name: "F".into(),
            params: vec!["x".into(), "Y".into()],
            body: Expr::a("x".into(), "Y".into()),
        };
        assert_eq!(ECMAScriptStyle(&f).to_string(), "F(x, Y) = x(Y)");
    }
}
