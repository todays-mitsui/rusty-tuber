use crate::expression::Expr;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct LazyKStyle<'a>(pub &'a Expr);

impl Display for LazyKStyle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_display() {
    assert_eq!(LazyKStyle(&"x".into()).to_string(), "x");

    assert_eq!(
        LazyKStyle(&Expr::a("x".into(), "y".into())).to_string(),
        "`xy"
    );

    assert_eq!(
        LazyKStyle(&Expr::a(Expr::a("x".into(), "y".into()), "z".into())).to_string(),
        "``xyz"
    );

    assert_eq!(
        LazyKStyle(&Expr::a("x".into(), Expr::a("y".into(), "z".into()))).to_string(),
        "`x`yz"
    );

    assert_eq!(
        LazyKStyle(&Expr::l("x".into(), "x".into())).to_string(),
        "^x.x"
    );

    assert_eq!(
        LazyKStyle(&Expr::l("x".into(), Expr::l("y".into(), "x".into()))).to_string(),
        "^x.^y.x"
    );
}
