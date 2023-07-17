use crate::expression::Expr;
use crate::identifier::Ident;

/// 定義済み関数を表現する
///
/// 関数とラムダ抽象はよく似ているが、関数が 0 以上の arity を持つ点で異なる
#[derive(Debug, PartialEq)]
pub struct Func {
    params: Vec<Ident>,
    body: Expr,
}

impl Func {
    pub fn new(params: Vec<Ident>, body: Expr) -> Func {
        Func { params, body }
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
}

pub fn i() -> Func {
    Func {
        params: vec![Ident::new("x")],
        body: Expr::v("x"),
    }
}

pub fn k() -> Func {
    Func {
        params: vec![Ident::new("x"), Ident::new("y")],
        body: Expr::v("x"),
    }
}

pub fn s() -> Func {
    Func {
        params: vec![
            Ident::new("x"),
            Ident::new("y"),
            Ident::new("z"),
        ],
        body: Expr::a(
            Expr::a(Expr::v("x"), Expr::v("z")),
            Expr::a(Expr::v("y"), Expr::v("z")),
        ),
    }
}