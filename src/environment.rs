use std::collections::HashMap;

use crate::function::Func;
use crate::function::{i, k, s};
use crate::identifier::Ident;

/// 定義済みの名前空間を表現する
///
/// 識別子と関数の組を保持する
#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    env: HashMap<Ident, Func>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            env: HashMap::new(),
        }
    }

    pub fn def(&mut self, func: Func) {
        self.env.insert(func.name().clone(), func);
    }

    pub fn get(&self, id: &Ident) -> Option<&Func> {
        self.env.get(id)
    }

    pub fn arity(&self, id: &Ident) -> Option<usize> {
        self.get(id).map(|f| f.arity())
    }

    #[allow(dead_code)]
    pub fn del(&mut self, id: &Ident) {
        self.env.remove(id);
    }

    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.env.len()
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::from(vec![i(), k(), s()])
    }
}

impl From<Vec<Func>> for Env {
    fn from(v: Vec<Func>) -> Self {
        let mut env = Env::new();
        for func in v {
            env.def(func);
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

        env.def(Func::new(
            Ident::new("i"),
            vec![Ident::new("x")],
            Expr::v("x"),
        ));
        env.def(Func::new(
            Ident::new("k"),
            vec![Ident::new("x"), Ident::new("y")],
            Expr::v("x"),
        ));
        env.def(Func::new(
            Ident::new("s"),
            vec![Ident::new("x"), Ident::new("y"), Ident::new("z")],
            Expr::a(
                Expr::a(Expr::v("x"), Expr::v("z")),
                Expr::a(Expr::v("y"), Expr::v("z")),
            ),
        ));

        assert_eq!(env.count(), 3);
        assert_eq!(env.arity(&Ident::new("i")), Some(1));
        assert_eq!(env.arity(&Ident::new("k")), Some(2));
        assert_eq!(env.arity(&Ident::new("s")), Some(3));
        assert_eq!(env.arity(&Ident::new("UNDEFINED")), None);

        env.del(&Ident::new("i"));
        assert_eq!(env.count(), 2);
        assert_eq!(env.arity(&Ident::new("i")), None);
    }

    #[test]
    fn test_env_from() {
        let i: Func = Func::new("i".into(), vec![Ident::new("x")], Expr::v("x"));
        let k: Func = Func::new(
            "k".into(),
            vec![Ident::new("x"), Ident::new("y")],
            Expr::v("x"),
        );
        let s: Func = Func::new(
            "s".into(),
            vec![Ident::new("x"), Ident::new("y"), Ident::new("z")],
            Expr::a(
                Expr::a(Expr::v("x"), Expr::v("z")),
                Expr::a(Expr::v("y"), Expr::v("z")),
            ),
        );

        let env: Env = Env::from(vec![i, k, s]);

        assert_eq!(env.count(), 3);
        assert_eq!(env.arity(&Ident::new("i")), Some(1));
        assert_eq!(env.arity(&Ident::new("k")), Some(2));
        assert_eq!(env.arity(&Ident::new("s")), Some(3));
        assert_eq!(env.arity(&Ident::new("UNDEFINED")), None);
    }
}
