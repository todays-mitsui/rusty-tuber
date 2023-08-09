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
            Expr::Symbol(_) => self,
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
        assert_eq!(Expr::Variable("x".into()).unlambda(), "x".into());
        assert_eq!(Expr::Symbol("x".into()).unlambda(), ":x".into());
        assert_eq!(
            Expr::a("x".into(), "y".into()).unlambda(),
            Expr::a("x".into(), "y".into())
        );
        assert_eq!(Expr::l("x".into(), "x".into()).unlambda(), "i".into());
        assert_eq!(
            Expr::l("x".into(), "y".into()).unlambda(),
            Expr::a("k".into(), "y".into())
        );
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
