use crate::expression::Expr;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct ECMAScriptStyle<'a>(pub &'a Expr);

impl Display for ECMAScriptStyle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", AnotherExpr::new(self.0).to_string())
    }
}

enum AnotherExpr<'a> {
    Variable(&'a str),
    Symbol(&'a str),
    Apply(Box<AnotherExpr<'a>>, Vec<AnotherExpr<'a>>),
    Lambda(Vec<&'a str>, Box<AnotherExpr<'a>>),
}

impl<'a> AnotherExpr<'a> {
    fn new(e: &'a Expr) -> AnotherExpr<'a> {
        match e {
            Expr::Variable(i) => {
                let label = i.label();
                AnotherExpr::Variable(label)
            }

            Expr::Symbol(i) => {
                let label = i.label();
                AnotherExpr::Symbol(label)
            }

            Expr::Apply { lhs, rhs } => {
                let e1 = AnotherExpr::new(lhs);
                let e2 = AnotherExpr::new(rhs);
                match e1 {
                    AnotherExpr::Apply(e1, mut es) => {
                        es.push(e2);
                        AnotherExpr::Apply(e1, es)
                    }
                    _ => AnotherExpr::Apply(Box::new(e1), vec![e2]),
                }
            }

            Expr::Lambda { param, body } => {
                let param = param.label();
                let body = AnotherExpr::new(body);
                match body {
                    AnotherExpr::Lambda(mut params, body) => {
                        params.push(param);
                        AnotherExpr::Lambda(params, body)
                    }
                    _ => AnotherExpr::Lambda(vec![&param], Box::new(body)),
                }
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            AnotherExpr::Variable(label) => format!("{}", label),
            AnotherExpr::Symbol(label) => format!(":{}", label),
            AnotherExpr::Apply(e, args) => {
                format!(
                    "{}({})",
                    e.to_string(),
                    args.iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            AnotherExpr::Lambda(params, body) => {
                if params.len() == 1 {
                    return format!("{} => {}", params[0], body.to_string());
                } else {
                    format!(
                        "({}) => {}",
                        params
                            .iter()
                            .map(|arg| arg.to_string())
                            .rev()
                            .collect::<Vec<_>>()
                            .join(", "),
                        body.to_string()
                    )
                }
            }
        }
    }
}

#[test]
fn test_display() {
    assert_eq!(ECMAScriptStyle(&"x".into()).to_string(), "x");

    assert_eq!(
        ECMAScriptStyle(&Expr::a("x".into(), "y".into())).to_string(),
        "x(y)"
    );

    assert_eq!(
        ECMAScriptStyle(&Expr::a(Expr::a("x".into(), "y".into()), "z".into())).to_string(),
        "x(y, z)"
    );

    assert_eq!(
        ECMAScriptStyle(&Expr::a("x".into(), Expr::a("y".into(), "z".into()))).to_string(),
        "x(y(z))"
    );

    assert_eq!(
        ECMAScriptStyle(&Expr::l("x".into(), "x".into())).to_string(),
        "x => x"
    );

    assert_eq!(
        ECMAScriptStyle(&Expr::l("x".into(), Expr::l("y".into(), "x".into()))).to_string(),
        "(x, y) => x"
    );
}
