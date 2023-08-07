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
            Expr::Apply { lhs, rhs } => Expr::a(
                Expr::a("s".into(), lhs.unlambda_(param)),
                rhs.unlambda_(param),
            ),
            Expr::Lambda { param: inner, body } => body.unlambda_(&inner).unlambda_(param),
        }
    }
}
