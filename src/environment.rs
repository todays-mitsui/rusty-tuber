use std::collections::HashMap;

use crate::function::Func;
use crate::identifier::Ident;

/// 定義済みの名前空間を表現する
///
/// 識別子と関数の組を保持する
pub struct Env {
    env: HashMap<Ident, Func>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            env: HashMap::new(),
        }
    }

    pub fn def(&mut self, id: Ident, func: Func) {
        self.env.insert(id, func);
    }

    pub fn get(&self, id: &Ident) -> Option<&Func> {
        self.env.get(id)
    }

    pub fn arity(&self, id: &Ident) -> Option<usize> {
        self.get(id).map(|f| f.arity())
    }

    pub fn del(&mut self, id: &Ident) {
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

impl From<Vec<(Ident, Func)>> for Env {
    fn from(v: Vec<(Ident, Func)>) -> Self {
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
    use crate::expression::Expr;
    use crate::function::Func;
    use crate::identifier::Ident;

    #[test]
    fn test_env_def() {
        let mut env = Env::new();

        assert_eq!(env.count(), 0);

        env.def(
            Ident::new("I"),
            Func::new(vec![Ident::new("x")], Expr::v("x")),
        );
        env.def(
            Ident::new("K"),
            Func::new(
                vec![Ident::new("x"), Ident::new("y")],
                Expr::v("x"),
            ),
        );
        env.def(
            Ident::new("S"),
            Func::new(
                vec![
                    Ident::new("x"),
                    Ident::new("y"),
                    Ident::new("z"),
                ],
                Expr::a(
                    Expr::a(Expr::v("x"), Expr::v("z")),
                    Expr::a(Expr::v("y"), Expr::v("z")),
                ),
            ),
        );

        assert_eq!(env.count(), 3);
        assert_eq!(env.arity(&Ident::new("I")), Some(1));
        assert_eq!(env.arity(&Ident::new("K")), Some(2));
        assert_eq!(env.arity(&Ident::new("S")), Some(3));
        assert_eq!(env.arity(&Ident::new("UNDEFINED")), None);

        env.del(&Ident::new("I"));
        assert_eq!(env.count(), 2);
        assert_eq!(env.arity(&Ident::new("I")), None);
    }

    #[test]
    fn test_env_from() {
        let i: Func = Func::new(vec![Ident::new("x")], Expr::v("x"));
        let k: Func = Func::new(
            vec![Ident::new("x"), Ident::new("y")],
            Expr::v("x"),
        );
        let s: Func = Func::new(
            vec![
                Ident::new("x"),
                Ident::new("y"),
                Ident::new("z"),
            ],
            Expr::a(
                Expr::a(Expr::v("x"), Expr::v("z")),
                Expr::a(Expr::v("y"), Expr::v("z")),
            ),
        );

        let env: Env = Env::from(vec![
            (Ident::new("I"), i),
            (Ident::new("K"), k),
            (Ident::new("S"), s),
        ]);

        assert_eq!(env.count(), 3);
        assert_eq!(env.arity(&Ident::new("I")), Some(1));
        assert_eq!(env.arity(&Ident::new("K")), Some(2));
        assert_eq!(env.arity(&Ident::new("S")), Some(3));
        assert_eq!(env.arity(&Ident::new("UNDEFINED")), None);
    }
}
