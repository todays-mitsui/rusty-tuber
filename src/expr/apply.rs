use super::Expr;
use crate::env::Env;

impl Expr {
    pub fn arity(&self, env: &Env) -> Option<usize> {
        match self {
            Expr::Lambda { .. } => Some(1),
            Expr::Variable(id) => env.arity(id),
            _ => None,
        }
    }

    /// TODO: Option<T> ではなく Result<T, E> を返すのが適切かも
    pub fn apply(&self, env: &Env, args: Vec<Expr>) -> Option<Expr> {
        match self {
            Expr::Lambda { param, body } => Some(body.clone().substitute(&param, &args[0])),

            Expr::Variable(id) => match env.get(&id) {
                Some(func) => Some(func.apply(args)),
                None => None,
            },

            _ => None,
        }
    }
}
