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

    pub fn apply(&self, env: &Env, args: Vec<Expr>) -> Expr {
        match self {
            Expr::Lambda { param, body } => body.clone().substitute(&param, &args[0]),

            Expr::Variable(id) => match env.get(&id) {
                Some(func) => func.apply(args),
                None => panic!("apply: unknown function"),
            },

            _ => panic!("apply: not a function"),
        }
    }
}
