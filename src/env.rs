use std::collections::HashMap;

use crate::expr::Identifier;
use crate::func::Func;

pub struct Env {
    env: HashMap<Identifier, Func>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            env: HashMap::new(),
        }
    }

    pub fn def(&mut self, id: Identifier, func: Func) {
        self.env.insert(id, func);
    }

    pub fn get(&self, id: &Identifier) -> Option<&Func> {
        self.env.get(id)
    }

    pub fn arity(&self, id: &Identifier) -> Option<usize> {
        self.get(id).map(|f| f.arity())
    }

    pub fn del(&mut self, id: &Identifier) {
        self.env.remove(id);
    }

    pub fn count(&self) -> usize {
        self.env.len()
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
    }
}

impl From<Vec<(Identifier, Func)>> for Env {
    fn from(v: Vec<(Identifier, Func)>) -> Self {
        let mut env = Env::new();
        for (id, func) in v {
            env.def(id, func);
        }
        env
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Expr, Identifier};
    use crate::func::Func;

    #[test]
    fn test_env_def() {
        let mut env = Env::new();

        assert_eq!(env.count(), 0);

        env.def(
            Identifier::new("I"),
            Func::new(vec![Identifier::new("x")], Expr::v("x")),
        );
        env.def(
            Identifier::new("K"),
            Func::new(
                vec![Identifier::new("x"), Identifier::new("y")],
                Expr::v("x"),
            ),
        );
        env.def(
            Identifier::new("S"),
            Func::new(
                vec![
                    Identifier::new("x"),
                    Identifier::new("y"),
                    Identifier::new("z"),
                ],
                Expr::a(
                    Expr::a(Expr::v("x"), Expr::v("z")),
                    Expr::a(Expr::v("y"), Expr::v("z")),
                ),
            ),
        );

        assert_eq!(env.count(), 3);
        assert_eq!(env.arity(&Identifier::new("I")), Some(1));
        assert_eq!(env.arity(&Identifier::new("K")), Some(2));
        assert_eq!(env.arity(&Identifier::new("S")), Some(3));
        assert_eq!(env.arity(&Identifier::new("UNDEFINED")), None);

        env.del(&Identifier::new("I"));
        assert_eq!(env.count(), 2);
        assert_eq!(env.arity(&Identifier::new("I")), None);
    }

    #[test]
    fn test_env_from() {
        let i: Func = Func::new(vec![Identifier::new("x")], Expr::v("x"));
        let k: Func = Func::new(
            vec![Identifier::new("x"), Identifier::new("y")],
            Expr::v("x"),
        );
        let s: Func = Func::new(
            vec![
                Identifier::new("x"),
                Identifier::new("y"),
                Identifier::new("z"),
            ],
            Expr::a(
                Expr::a(Expr::v("x"), Expr::v("z")),
                Expr::a(Expr::v("y"), Expr::v("z")),
            ),
        );

        let env: Env = Env::from(vec![
            (Identifier::new("I"), i),
            (Identifier::new("K"), k),
            (Identifier::new("S"), s),
        ]);

        assert_eq!(env.count(), 3);
        assert_eq!(env.arity(&Identifier::new("I")), Some(1));
        assert_eq!(env.arity(&Identifier::new("K")), Some(2));
        assert_eq!(env.arity(&Identifier::new("S")), Some(3));
        assert_eq!(env.arity(&Identifier::new("UNDEFINED")), None);
    }
}
