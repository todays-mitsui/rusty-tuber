use crate::expression::Expr;
use crate::identifier::Ident;
use std::collections::HashSet;

type FreeVars<'a> = HashSet<&'a Ident>;

pub fn free_vars(expr: &Expr) -> FreeVars {
    let mut vars: FreeVars = HashSet::new();
    free_vars_impl(expr, &mut vars);
    vars
}

fn free_vars_impl<'a>(expr: &'a Expr, vars: &mut FreeVars<'a>) {
    match expr {
        Expr::Variable(id) => {
            vars.insert(id);
        }
        Expr::Symbol(_) => {}
        Expr::Apply { lhs, rhs } => {
            free_vars_impl(lhs.as_ref(), vars);
            free_vars_impl(rhs.as_ref(), vars);
        }
        Expr::Lambda { param, body } => {
            let mut body_vars: FreeVars = HashSet::new();
            free_vars_impl(body.as_ref(), &mut body_vars);
            for var in body_vars {
                if var != param {
                    vars.insert(var);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_vars() {
        assert_eq!(
            free_vars(&Expr::Variable("x".into())),
            vec![&"x".into()].into_iter().collect()
        );
        assert_eq!(free_vars(&Expr::Symbol("x".into())), HashSet::new());
        assert_eq!(
            free_vars(&Expr::a("x".into(), "y".into())),
            vec![&"x".into(), &"y".into()].into_iter().collect()
        );
        assert_eq!(free_vars(&Expr::l("x".into(), "x".into())), HashSet::new());
        assert_eq!(
            free_vars(&Expr::l("x".into(), "y".into())),
            vec![&"y".into()].into_iter().collect()
        );
        assert_eq!(
            free_vars(&Expr::l(
                "x".into(),
                Expr::l("y".into(), Expr::a("x".into(), "y".into()))
            )),
            HashSet::new()
        );
    }
}
