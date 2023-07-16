use std::collections::HashSet;

use super::{Expr, Identifier};
use crate::expr::free_vars::FreeVars;

impl Expr {
    /// 指定した識別子を別の式で置き換えた新しい式を得る
    ///
    /// 単純な置換だけでなく、ラムダ抽象の中で束縛されている束縛変数と自由変数の衝突を避けるため
    /// 束縛変数のリネームを行う
    ///
    /// 具体例はテストコードを参照
    pub fn substitute(self, param: &Identifier, arg: &Expr) -> Expr {
        let mut vars = HashSet::new();
        self.substitute_impl(param, arg, &arg.free_vars(), &mut vars)
    }

    fn substitute_impl(
        self,
        param: &Identifier,
        arg: &Expr,
        free_vars: &FreeVars,
        bound_vars: &mut HashSet<Identifier>,
    ) -> Expr {
        match self {
            Expr::Variable(id) => {
                if &id == param {
                    arg.clone()
                } else {
                    Expr::Variable(id)
                }
            }

            Expr::Symbol(_) => self,

            Expr::Apply { lhs, rhs } => Expr::Apply {
                lhs: Box::new(lhs.substitute_impl(param, arg, free_vars, &mut bound_vars.clone())),
                rhs: Box::new(rhs.substitute_impl(param, arg, free_vars, &mut bound_vars.clone())),
            },

            Expr::Lambda { param: p, body } => {
                if &p == param {
                    Expr::Lambda { param: p, body }
                } else if free_vars.contains(&p) {
                    let new_param: Identifier = p.new_name(bound_vars);
                    bound_vars.insert(new_param.clone());

                    let mut new_body = body.clone();
                    new_body.rename_var(&p, &new_param);

                    Expr::Lambda {
                        param: new_param,
                        body: Box::new(new_body.substitute_impl(param, arg, free_vars, bound_vars)),
                    }
                } else {
                    bound_vars.insert(p.clone());
                    Expr::Lambda {
                        param: p,
                        body: Box::new(body.substitute_impl(param, arg, free_vars, bound_vars)),
                    }
                }
            }
        }
    }

    /// 式の中の自由変数を別の識別子に置き換える
    fn rename_var(&mut self, old: &Identifier, new: &Identifier) {
        match self {
            Expr::Variable(id) => {
                if id == old {
                    *id = new.clone();
                }
            }

            Expr::Symbol(_) => {}

            Expr::Apply { lhs, rhs } => {
                lhs.rename_var(old, new);
                rhs.rename_var(old, new);
            }

            Expr::Lambda { param, body } => {
                if param != old {
                    body.rename_var(old, new);
                }
                // 自由変数としての old のみ new に置き換えたい
                // old が束縛変数の識別子と一致する場合、そのラムダ抽象の中に自由変数としての old は
                // 存在しないことが確定するので、その時点で再起を打ち切っていい
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_substitute() {
        // ^z.x [x := y] => ^z.y
        assert_eq!(
            Expr::l("z".into(), "x".into()).substitute(&"x".into(), &"y".into()),
            Expr::l("z".into(), "y".into())
        );

        // ^Y.^y.`xY [x := y] => ^Y.^Y0.`yY
        assert_eq!(
            Expr::l(
                "Y".into(),
                Expr::l("y".into(), Expr::a("x".into(), "Y".into()))
            )
            .substitute(&"x".into(), &"y".into()),
            Expr::l(
                "Y".into(),
                Expr::l("Y0".into(), Expr::a("y".into(), "Y".into()))
            )
        );
    }

    #[test]
    /// ラムダ抽象の中で束縛されている変数は置換されない
    fn test_rename_var_1() {
        let mut expr = Expr::l("x".into(), Expr::a("x".into(), "y".into()));
        let expected = Expr::l("x".into(), Expr::a("x".into(), "y".into()));

        expr.rename_var(&"x".into(), &"a".into());

        assert_eq!(expr, expected);
    }

    #[test]
    /// 置換はラムダ抽象の中にまで渡って再起的に行われる
    fn test_rename_var_2() {
        let mut expr = Expr::l("x".into(), Expr::a("x".into(), "y".into()));
        let expected = Expr::l("x".into(), Expr::a("x".into(), "a".into()));

        expr.rename_var(&"y".into(), &"a".into());

        assert_eq!(expr, expected);
    }

    #[test]
    /// 置換は左右の枝に渡って再起的に行われる
    fn test_rename_var_3() {
        let mut expr = Expr::a(Expr::a("x".into(), "y".into()), Expr::a("y".into(), "x".into()));
        let expected = Expr::a(Expr::a("a".into(), "y".into()), Expr::a("y".into(), "a".into()));

        expr.rename_var(&"x".into(), &"a".into());

        assert_eq!(expr, expected);
    }


    #[test]
    /// 変数とシンボルは区別される
    /// :x が x によって置換されることはない
    fn test_rename_var_4() {
        let mut expr = Expr::a(Expr::a(":x".into(), "y".into()), Expr::a("y".into(), ":x".into()));
        let expected = Expr::a(Expr::a(":x".into(), "y".into()), Expr::a("y".into(), ":x".into()));

        expr.rename_var(&"x".into(), &"a".into());

        assert_eq!(expr, expected);
    }
}
