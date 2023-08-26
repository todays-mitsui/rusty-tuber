use crate::function::Func;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct LazyKStyle<'a>(pub &'a Func);

impl Display for LazyKStyle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;

    #[test]
    fn test_to_string_1() {
        let f = Func {
            name: "f".into(),
            params: vec!["x".into(), "y".into()],
            body: Expr::a("x".into(), "y".into()),
        };
        assert_eq!(LazyKStyle(&f).to_string(), "``fxy = `xy");
    }

    #[test]
    fn test_to_string_2() {
        let f = Func {
            name: "F".into(),
            params: vec!["X".into(), "Y".into()],
            body: Expr::a("X".into(), "Y".into()),
        };
        assert_eq!(LazyKStyle(&f).to_string(), "``F X Y = `X Y");
    }

    #[test]
    fn test_to_string_3() {
        let f = Func {
            name: "F".into(),
            params: vec!["x".into(), "Y".into()],
            body: Expr::a("x".into(), "Y".into()),
        };
        assert_eq!(LazyKStyle(&f).to_string(), "``FxY = `xY");
    }
}
