use std::collections::HashMap;

use crate::function::Func;
use crate::function::{i, k, s};
use crate::identifier::Ident;

/// 定義済みの名前空間を表現する
///
/// 識別子と関数の組を保持する
#[derive(Debug, Clone, PartialEq)]
pub struct Context(HashMap<Ident, Func>);

impl Context {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn def(&mut self, func: Func) {
        self.0.insert(func.name().clone(), func);
    }

    pub fn get(&self, id: &Ident) -> Option<&Func> {
        self.0.get(id)
    }

    pub fn arity(&self, id: &Ident) -> Option<usize> {
        self.get(id).map(|f| f.arity())
    }

    #[allow(dead_code)]
    pub fn del(&mut self, id: &Ident) {
        self.0.remove(id);
    }

    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn for_each(&self, callback: impl Fn(&Ident, &Func)) {
        for (i, f) in &self.0 {
            callback(i, f);
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::from(vec![i(), k(), s()])
    }
}

impl From<Vec<Func>> for Context {
    fn from(v: Vec<Func>) -> Self {
        let mut context = Context::new();
        for func in v {
            context.def(func);
        }
        context
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;
    use crate::function::Func;
    use crate::identifier::Ident;

    #[test]
    fn test_context_def() {
        let mut context = Context::new();

        assert_eq!(context.count(), 0);

        context.def(Func::new(
            Ident::new("i"),
            vec![Ident::new("x")],
            Expr::v("x"),
        ));
        context.def(Func::new(
            Ident::new("k"),
            vec![Ident::new("x"), Ident::new("y")],
            Expr::v("x"),
        ));
        context.def(Func::new(
            Ident::new("s"),
            vec![Ident::new("x"), Ident::new("y"), Ident::new("z")],
            Expr::a(
                Expr::a(Expr::v("x"), Expr::v("z")),
                Expr::a(Expr::v("y"), Expr::v("z")),
            ),
        ));

        assert_eq!(context.count(), 3);
        assert_eq!(context.arity(&Ident::new("i")), Some(1));
        assert_eq!(context.arity(&Ident::new("k")), Some(2));
        assert_eq!(context.arity(&Ident::new("s")), Some(3));
        assert_eq!(context.arity(&Ident::new("UNDEFINED")), None);

        context.del(&Ident::new("i"));
        assert_eq!(context.count(), 2);
        assert_eq!(context.arity(&Ident::new("i")), None);
    }

    #[test]
    fn test_context_from() {
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

        let context: Context = Context::from(vec![i, k, s]);

        assert_eq!(context.count(), 3);
        assert_eq!(context.arity(&Ident::new("i")), Some(1));
        assert_eq!(context.arity(&Ident::new("k")), Some(2));
        assert_eq!(context.arity(&Ident::new("s")), Some(3));
        assert_eq!(context.arity(&Ident::new("UNDEFINED")), None);
    }
}
