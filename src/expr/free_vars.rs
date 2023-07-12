use std::collections::HashSet;

use crate::expr::{Expr, Identifier};

#[derive(Debug)]
pub struct FreeVars {
    pub vars: HashSet<Identifier>,
}

impl FreeVars {
    pub fn count(&self) -> usize {
        self.vars.len()
    }

    pub fn contains(&self, id: &Identifier) -> bool {
        self.vars.contains(id)
    }
}

impl From<HashSet<Identifier>> for FreeVars {
    fn from(vars: HashSet<Identifier>) -> FreeVars {
        FreeVars { vars }
    }
}

impl Expr {
    pub fn free_vars(&self) -> FreeVars {
        let mut vars = HashSet::new();
        self.free_vars_impl(&mut vars);
        FreeVars { vars }
    }

    fn free_vars_impl(&self, vars: &mut HashSet<Identifier>) {
        match self {
            Expr::Variable(id) => {
                vars.insert(id.clone());
            }
            Expr::Symbol(_) => {}
            Expr::Apply { lhs, rhs } => {
                lhs.free_vars_impl(vars);
                rhs.free_vars_impl(vars);
            }
            Expr::Lambda { param, body } => {
                let mut body_vars = HashSet::new();
                body.free_vars_impl(&mut body_vars);
                for var in body_vars {
                    if &var != param {
                        vars.insert(var);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{Expr, Identifier};

    #[test]
    fn test_free_vars() {
        assert_eq!(Expr::v("a").free_vars().count(), 1);
        assert!(Expr::v("a").free_vars().contains(&Identifier::new("a")));

        assert_eq!(Expr::s("a").free_vars().count(), 0);

        assert_eq!(Expr::a(Expr::v("a"), Expr::v("b")).free_vars().count(), 2);
        assert!(Expr::a(Expr::v("a"), Expr::v("b"))
            .free_vars()
            .contains(&Identifier::new("a")));
        assert!(Expr::a(Expr::v("a"), Expr::v("b"))
            .free_vars()
            .contains(&Identifier::new("b")));

        assert_eq!(
            Expr::l(Identifier::new("a"), Expr::v("a"))
                .free_vars()
                .count(),
            0
        );
        assert!(!Expr::l(Identifier::new("a"), Expr::v("a"))
            .free_vars()
            .contains(&Identifier::new("a")));

        assert_eq!(
            Expr::l(Identifier::new("a"), Expr::v("b"))
                .free_vars()
                .count(),
            1
        );
        assert!(Expr::l(Identifier::new("a"), Expr::v("b"))
            .free_vars()
            .contains(&Identifier::new("b")));
    }
}
