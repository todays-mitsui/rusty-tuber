use crate::context::Context;
use crate::expression::Expr;
use crate::function::Func;

impl Default for Context {
    fn default() -> Self {
        Context::from(vec![
            Func::new("i".into(), vec!["x".into()], "x".into()),
            Func::new("k".into(), vec!["x".into(), "y".into()], "x".into()),
            Func::new(
                "s".into(),
                vec!["x".into(), "y".into(), "z".into()],
                Expr::a(
                    Expr::a("x".into(), "z".into()),
                    Expr::a("y".into(), "z".into()),
                ),
            ),
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
                Expr::l(
                    "f".into(),
                    Expr::a(Expr::a("f".into(), "x".into()), "y".into()),
                ),
            ),
            Func::new(
                "CAR".into(),
                vec!["x".into()],
                Expr::a("x".into(), "TRUE".into()),
            ),
            Func::new(
                "CDR".into(),
                vec!["x".into()],
                Expr::a("x".into(), "FALSE".into()),
            ),
            Func::new("NIL".into(), vec![], "FALSE".into()),
            Func::new(
                "IS_NIL".into(),
                vec!["x".into()],
                Expr::a(
                    Expr::a("x".into(), Expr::l("_".into(), "FALSE".into())),
                    "TRUE".into(),
                ),
            ),
            Func::new(
                "Y".into(),
                vec!["f".into()],
                Expr::a(
                    Expr::l(
                        "x".into(),
                        Expr::a("f".into(), Expr::a("x".into(), "x".into())),
                    ),
                    Expr::l(
                        "x".into(),
                        Expr::a("f".into(), Expr::a("x".into(), "x".into())),
                    ),
                ),
            ),
            Func::new(
                "Z".into(),
                vec!["f".into()],
                Expr::a(
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::l(
                                "y".into(),
                                Expr::a(Expr::a("x".into(), "x".into()), "y".into()),
                            ),
                        ),
                    ),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::l(
                                "y".into(),
                                Expr::a(Expr::a("x".into(), "x".into()), "y".into()),
                            ),
                        ),
                    ),
                ),
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
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            Expr::a(
                                Expr::a(
                                    "n".into(),
                                    Expr::l(
                                        "g".into(),
                                        Expr::l(
                                            "h".into(),
                                            Expr::a("h".into(), Expr::a("g".into(), "f".into())),
                                        ),
                                    ),
                                ),
                                Expr::l("u".into(), "x".into()),
                            ),
                            Expr::l("u".into(), "u".into()),
                        ),
                    ),
                ),
            ),
            Func::new(
                "SUB".into(),
                vec!["m".into(), "n".into()],
                Expr::a(Expr::a("n".into(), "PRED".into()), "m".into()),
            ),
            Func::new(
                "GTE".into(),
                vec!["m".into(), "n".into()],
                Expr::a(
                    "IS_ZERO".into(),
                    Expr::a(Expr::a("SUB".into(), "n".into()), "m".into()),
                ),
            ),
            Func::new(
                "LTE".into(),
                vec!["m".into(), "n".into()],
                Expr::a(
                    "IS_ZERO".into(),
                    Expr::a(Expr::a("SUB".into(), "m".into()), "n".into()),
                ),
            ),
            Func::new(
                "EQ".into(),
                vec!["m".into(), "n".into()],
                Expr::a(
                    Expr::a(
                        "AND".into(),
                        Expr::a(Expr::a("GTE".into(), "m".into()), "n".into()),
                    ),
                    Expr::a(Expr::a("LTE".into(), "m".into()), "n".into()),
                ),
            ),
            Func::new(
                "0".into(),
                vec![],
                Expr::l("f".into(), Expr::l("x".into(), "x".into())),
            ),
            Func::new(
                "1".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l("x".into(), Expr::a("f".into(), "x".into())),
                ),
            ),
            Func::new(
                "2".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a("f".into(), Expr::a("f".into(), "x".into())),
                    ),
                ),
            ),
            Func::new(
                "3".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a("f".into(), Expr::a("f".into(), "x".into())),
                        ),
                    ),
                ),
            ),
            Func::new(
                "4".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a(
                                "f".into(),
                                Expr::a("f".into(), Expr::a("f".into(), "x".into())),
                            ),
                        ),
                    ),
                ),
            ),
            Func::new(
                "5".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a(
                                "f".into(),
                                Expr::a(
                                    "f".into(),
                                    Expr::a("f".into(), Expr::a("f".into(), "x".into())),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            Func::new(
                "6".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a(
                                "f".into(),
                                Expr::a(
                                    "f".into(),
                                    Expr::a(
                                        "f".into(),
                                        Expr::a("f".into(), Expr::a("f".into(), "x".into())),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            Func::new(
                "7".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a(
                                "f".into(),
                                Expr::a(
                                    "f".into(),
                                    Expr::a(
                                        "f".into(),
                                        Expr::a(
                                            "f".into(),
                                            Expr::a("f".into(), Expr::a("f".into(), "x".into())),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            Func::new(
                "8".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a(
                                "f".into(),
                                Expr::a(
                                    "f".into(),
                                    Expr::a(
                                        "f".into(),
                                        Expr::a(
                                            "f".into(),
                                            Expr::a(
                                                "f".into(),
                                                Expr::a(
                                                    "f".into(),
                                                    Expr::a("f".into(), "x".into()),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            Func::new(
                "9".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a(
                                "f".into(),
                                Expr::a(
                                    "f".into(),
                                    Expr::a(
                                        "f".into(),
                                        Expr::a(
                                            "f".into(),
                                            Expr::a(
                                                "f".into(),
                                                Expr::a(
                                                    "f".into(),
                                                    Expr::a(
                                                        "f".into(),
                                                        Expr::a("f".into(), "x".into()),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            Func::new(
                "10".into(),
                vec![],
                Expr::l(
                    "f".into(),
                    Expr::l(
                        "x".into(),
                        Expr::a(
                            "f".into(),
                            Expr::a(
                                "f".into(),
                                Expr::a(
                                    "f".into(),
                                    Expr::a(
                                        "f".into(),
                                        Expr::a(
                                            "f".into(),
                                            Expr::a(
                                                "f".into(),
                                                Expr::a(
                                                    "f".into(),
                                                    Expr::a(
                                                        "f".into(),
                                                        Expr::a(
                                                            "f".into(),
                                                            Expr::a("f".into(), "x".into()),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ])
    }
}
