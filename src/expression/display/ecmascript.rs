use crate::expression::Expr;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct ECMAScriptStyle<'a>(pub &'a Expr);

impl Display for ECMAScriptStyle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_display() {
    assert_eq!(ECMAScriptStyle(&"x".into()).to_string(), "x");

    assert_eq!(
        ECMAScriptStyle(&Expr::a("x".into(), "y".into())).to_string(),
        "x(y)"
    );

    assert_eq!(
        ECMAScriptStyle(&Expr::a(Expr::a("x".into(), "y".into()), "z".into())).to_string(),
        "x(y, z)"
    );

    assert_eq!(
        ECMAScriptStyle(&Expr::a("x".into(), Expr::a("y".into(), "z".into()))).to_string(),
        "x(y(z))"
    );

    assert_eq!(
        ECMAScriptStyle(&Expr::l("x".into(), "x".into())).to_string(),
        "x => x"
    );

    assert_eq!(
        ECMAScriptStyle(&Expr::l("x".into(), Expr::l("y".into(), "x".into()))).to_string(),
        "x => y => x"
    );
}
