use crate::expression::free_vars::free_vars;
use crate::expression::Expr;
use crate::identifier::Ident;

impl Expr {
    pub fn unlambda(self) -> Expr {
        match self {
            Expr::Variable(_) => self,
            Expr::Symbol(_) => self,
            Expr::Apply { lhs, rhs } => Expr::a(lhs.unlambda(), rhs.unlambda()),
            Expr::Lambda { param, body } => body.unlambda_(&param),
        }
    }

    fn unlambda_(self, param: &Ident) -> Expr {
        match self {
            Expr::Variable(id) if &id == param => "i".into(),
            Expr::Variable(id) => Expr::a("k".into(), Expr::Variable(id)),
            Expr::Symbol(_) => Expr::a("k".into(), self),
            Expr::Apply { .. } if !free_vars(&self).contains(param) => Expr::a("k".into(), self),
            Expr::Apply { lhs, rhs } => match rhs.as_ref() {
                Expr::Variable(id) if id == param && !free_vars(lhs.as_ref()).contains(param) => {
                    *lhs
                }
                _ => Expr::a(
                    Expr::a("s".into(), lhs.unlambda_(param)),
                    rhs.unlambda_(param),
                ),
            },
            Expr::Lambda { param: inner, body } => body.unlambda_(&inner).unlambda_(param),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlambda() {
        // x == x
        assert_eq!(Expr::Variable("x".into()).unlambda(), "x".into());

        // :x = :x
        assert_eq!(Expr::Symbol("x".into()).unlambda(), ":x".into());

        // `xy == `xy
        assert_eq!(
            Expr::a("x".into(), "y".into()).unlambda(),
            Expr::a("x".into(), "y".into())
        );

        // ^x.x == i
        assert_eq!(Expr::l("x".into(), "x".into()).unlambda(), "i".into());

        // ^x.:x == `k:x
        assert_eq!(
            Expr::l("x".into(), ":x".into()).unlambda(),
            Expr::a("k".into(), ":x".into())
        );

        // ^x.y == `ky
        assert_eq!(
            Expr::l("x".into(), "y".into()).unlambda(),
            Expr::a("k".into(), "y".into())
        );

        // ^x.:y == `k:y
        assert_eq!(
            Expr::l("x".into(), ":y".into()).unlambda(),
            Expr::a("k".into(), ":y".into())
        );

        // ^x.`yx == y
        assert_eq!(
            Expr::l("x".into(), Expr::a("y".into(), "x".into())).unlambda(),
            "y".into()
        );

        // ^x.`y:x == `k`y:x
        assert_eq!(
            Expr::l("x".into(), Expr::a("y".into(), ":x".into())).unlambda(),
            Expr::a("k".into(), Expr::a("y".into(), ":x".into()))
        );

        // ^x.`xy == ``si`ky
        assert_eq!(
            Expr::l("x".into(), Expr::a("x".into(), "y".into())).unlambda(),
            Expr::a(
                Expr::a("s".into(), "i".into()),
                Expr::a("k".into(), "y".into())
            )
        );

        // ^x.`:xy == `k`:xy
        assert_eq!(
            Expr::l("x".into(), Expr::a(":x".into(), "y".into())).unlambda(),
            Expr::a("k".into(), Expr::a(":x".into(), "y".into()))
        );

        // ^x.`yz == `k`yz
        assert_eq!(
            Expr::l("x".into(), Expr::a("y".into(), "z".into())).unlambda(),
            Expr::a("k".into(), Expr::a("y".into(), "z".into()))
        );

        // ^x.^y.`xy == i
        assert_eq!(
            Expr::l(
                "x".into(),
                Expr::l("y".into(), Expr::a("x".into(), "y".into()))
            )
            .unlambda(),
            "i".into()
        );
    }
}
