use std::fmt::Display;

use crate::expression::Expr;
use crate::identifier::Ident;

/// 定義済み関数を表現する
///
/// 関数とラムダ抽象はよく似ているが、関数が 0 以上の arity を持つ点で異なる
#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    name: Ident,
    params: Vec<Ident>,
    body: Expr,
}

impl Func {
    pub fn new(name: Ident, params: Vec<Ident>, body: Expr) -> Func {
        Func { name, params, body }
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    /// 関数の引数の個数
    ///
    /// 0 以上の整数値を返す
    pub fn arity(&self) -> usize {
        self.params.len()
    }

    /// 関数に引数を与え評価した結果を返す
    pub fn apply(&self, args: Vec<Expr>) -> Expr {
        let mut body = self.body.clone();
        for (param, arg) in self.params.iter().zip(args) {
            body = body.substitute(param, &arg);
        }
        body
    }

    pub fn params(&self) -> &Vec<Ident> {
        &self.params
    }

    pub fn body(&self) -> &Expr {
        &self.body
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: ちゃんとする
        let mut lhs = Expr::Variable(self.name().clone());
        for i in self.params() {
            lhs = Expr::a(lhs, Expr::Variable(i.clone()));
        }
        write!(f, "{} = {}", lhs, self.body())
    }
}

/// i := ^x.x
pub fn i() -> Func {
    Func {
        name: "i".into(),
        params: vec![Ident::new("x")],
        body: Expr::v("x"),
    }
}

/// k := ^x.^y.x
pub fn k() -> Func {
    Func {
        name: "k".into(),
        params: vec![Ident::new("x"), Ident::new("y")],
        body: Expr::v("x"),
    }
}

/// s := ^x.^y.^z.``xz`yz
pub fn s() -> Func {
    Func {
        name: "s".into(),
        params: vec![Ident::new("x"), Ident::new("y"), Ident::new("z")],
        body: Expr::a(
            Expr::a(Expr::v("x"), Expr::v("z")),
            Expr::a(Expr::v("y"), Expr::v("z")),
        ),
    }
}
