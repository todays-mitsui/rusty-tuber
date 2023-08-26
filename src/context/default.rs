use crate::context::Context;
use crate::expression::Expr;
use crate::function::Func;
use crate::function::{i, k, s};
use crate::parser::expression::ecmascript::expr;
#[allow(unused_imports)]
use combine::EasyParser;

impl Default for Context {
    fn default() -> Self {
        Context::from(vec![
            i(),
            k(),
            s(),
            Func::new(
                "TRUE".into(),
                vec![],
                Expr::l("x".into(), Expr::l("y".into(), "x".into())),
            ),
            Func::new(
                "FALSE".into(),
                vec![],
                Expr::l("x".into(), Expr::l("y".into(), "y".into())),
            ),
            Func::new(
                "IF".into(),
                vec!["PRED".into(), "THEN".into(), "ELSE".into()],
                Expr::a(Expr::a("PRED".into(), "THEN".into()), "ELSE".into()),
            ),
            Func::new(
                "NOT".into(),
                vec!["x".into()],
                Expr::a(Expr::a("x".into(), "FALSE".into()), "TRUE".into()),
            ),
            Func::new(
                "AND".into(),
                vec!["x".into(), "y".into()],
                Expr::a(Expr::a("x".into(), "y".into()), "FALSE".into()),
            ),
            Func::new(
                "OR".into(),
                vec!["x".into(), "y".into()],
                Expr::a(Expr::a("x".into(), "TRUE".into()), "y".into()),
            ),
            Func::new(
                "XOR".into(),
                vec!["x".into(), "y".into()],
                Expr::a(
                    Expr::a("x".into(), Expr::a("NOT".into(), "y".into())),
                    "y".into(),
                ),
            ),
            Func::new(
                "CONS".into(),
                vec!["x".into(), "y".into()],
                expr().easy_parse("f=>f(x, y)").unwrap().0,
            ),
            Func::new(
                "CAR".into(),
                vec!["x".into()],
                expr().easy_parse("x(TRUE)").unwrap().0,
            ),
            Func::new(
                "CDR".into(),
                vec!["x".into()],
                expr().easy_parse("x(FALSE)").unwrap().0,
            ),
            Func::new("NIL".into(), vec![], "FALSE".into()),
            Func::new(
                "IS_NIL".into(),
                vec![],
                expr().easy_parse("x(_=>FALSE, TRUE)").unwrap().0,
            ),
            Func::new(
                "IS_ZERO".into(),
                vec!["n".into()],
                Expr::a(
                    Expr::a("n".into(), Expr::l("_".into(), "FALSE".into())),
                    "TRUE".into(),
                ),
            ),
            Func::new(
                "SUCC".into(),
                vec!["n".into()],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a(Expr::a("n".into(), "f".into()), "x".into()),
                        ),
                    ),
                ),
            ),
            Func::new(
                "ADD".into(),
                vec!["m".into(), "n".into()],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            Expr::a("m".into(), "f".into()),
                            Expr::a(Expr::a("n".into(), "f".into()), "x".into()),
                        ),
                    ),
                ),
            ),
            Func::new(
                "MUL".into(),
                vec!["m".into(), "n".into()],
                Expr::l(
                    "f".into(),
                    Expr::a("m".into(), Expr::a("n".into(), "f".into())),
                ),
            ),
            Func::new(
                "POW".into(),
                vec!["m".into(), "n".into()],
                Expr::a("n".into(), "m".into()),
            ),
            Func::new(
                "PRED".into(),
                vec!["n".into()],
                expr()
                    .easy_parse("f=>x=>n(g=>h=>h(g(f)))(u=>x)(u=>u)")
                    .unwrap()
                    .0,
            ),
            Func::new(
                "SUB".into(),
                vec!["m".into(), "n".into()],
                expr().easy_parse("n(pred, m)").unwrap().0,
            ),
            Func::new(
                "GTE".into(),
                vec!["m".into(), "n".into()],
                Expr::a(
                    "IS_ZERO".into(),
                    Expr::a("SUB".into(), Expr::a("m".into(), "n".into())),
                ),
            ),
            Func::new(
                "LTE".into(),
                vec!["m".into(), "n".into()],
                Expr::a(
                    "IS_ZERO".into(),
                    Expr::a("SUB".into(), Expr::a("n".into(), "m".into())),
                ),
            ),
            Func::new(
                "EQ".into(),
                vec!["m".into(), "n".into()],
                expr().easy_parse("AND(GTE(m,n), LTE(m, n))").unwrap().0,
            ),
            Func::new("0".into(), vec![], expr().easy_parse("f=>x=>x").unwrap().0),
            Func::new(
                "1".into(),
                vec![],
                expr().easy_parse("f=>x=>f(x)").unwrap().0,
            ),
            Func::new(
                "2".into(),
                vec![],
                expr().easy_parse("f=>x=>f(f(x))").unwrap().0,
            ),
            Func::new(
                "3".into(),
                vec![],
                expr().easy_parse("f=>x=>f(f(f(x)))").unwrap().0,
            ),
            Func::new("4".into(), vec![], expr().easy_parse("2(2)").unwrap().0),
            Func::new(
                "5".into(),
                vec![],
                expr().easy_parse("f=>x=>f(f(f(f(f(x)))))").unwrap().0,
            ),
            Func::new(
                "6".into(),
                vec![],
                expr().easy_parse("MUL(2, 3)").unwrap().0,
            ),
            Func::new(
                "7".into(),
                vec![],
                expr().easy_parse("f=>x=>f(f(f(f(f(f(f(x)))))))").unwrap().0,
            ),
            Func::new("8".into(), vec![], expr().easy_parse("3(2)").unwrap().0),
            Func::new("9".into(), vec![], expr().easy_parse("2(3)").unwrap().0),
            Func::new(
                "10".into(),
                vec![],
                expr().easy_parse("MUL(2, 5)").unwrap().0,
            ),
        ])
    }
}
