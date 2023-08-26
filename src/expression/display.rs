pub mod lazy_k;

use crate::expression::Expr;
use regex::Regex;
use std::fmt::Display;

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(&mut tokens(self)))
    }
}

#[derive(Debug, PartialEq)]
enum Ident<'a> {
    Variable(&'a str),
    Symbol(&'a str),
}

impl Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ident::Variable(label) => write!(f, "{}", label),
            Ident::Symbol(label) => write!(f, ":{}", label),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    UpperIdent(Ident<'a>),
    LowerIdent(Ident<'a>),
    Apply,
    Lambda,
    Dot,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::UpperIdent(i) => write!(f, "{}", i),
            Token::LowerIdent(i) => write!(f, "{}", i),
            Token::Apply => write!(f, "`"),
            Token::Lambda => write!(f, "^"),
            Token::Dot => write!(f, "."),
        }
    }
}

fn is_upper_ident(s: &str) -> bool {
    let regex_upper_ident: Regex = Regex::new(r"^[A-Z_]+$").unwrap();
    regex_upper_ident.is_match(s)
}

fn tokens<'a>(expr: &'a Expr) -> Vec<Token<'a>> {
    match expr {
        Expr::Variable(i) => {
            let label = i.label();
            if is_upper_ident(label) {
                vec![Token::UpperIdent(Ident::Variable(label))]
            } else {
                vec![Token::LowerIdent(Ident::Variable(label))]
            }
        }

        Expr::Symbol(i) => {
            let label = i.label();
            if is_upper_ident(label) {
                vec![Token::UpperIdent(Ident::Symbol(label))]
            } else {
                vec![Token::LowerIdent(Ident::Symbol(label))]
            }
        }

        Expr::Apply { lhs, rhs } => {
            let mut lhs = tokens(lhs);
            let mut rhs = tokens(rhs);
            rhs.append(&mut lhs);
            rhs.push(Token::Apply);
            rhs
        }

        Expr::Lambda { param, body } => {
            let mut body = tokens(body);
            body.push(Token::Dot);
            let label = param.label();
            if is_upper_ident(label) {
                body.push(Token::UpperIdent(Ident::Variable(label)));
            } else {
                body.push(Token::LowerIdent(Ident::Variable(label)));
            }
            body.push(Token::Lambda);
            body
        }
    }
}

fn to_string(tokens: &mut Vec<Token>) -> String {
    let mut str = String::new();
    while tokens.len() > 0 {
        match tokens.len() {
            1 => {
                let t = tokens.pop().unwrap();
                str.push_str(&format!("{}", t))
            }
            _ => {
                let t1 = tokens.pop().unwrap();
                let t2 = tokens.pop().unwrap();
                match (t1, &t2) {
                    (Token::UpperIdent(ident1), Token::UpperIdent(Ident::Variable(_))) => {
                        str.push_str(format!("{} ", ident1).as_str());
                        tokens.push(t2);
                    }
                    (t1, _) => {
                        str.push_str(format!("{}", t1).as_str());
                        tokens.push(t2);
                    }
                }
            }
        }
    }
    str
}

#[test]
fn test_to_string() {
    assert_eq!(Expr::a("x".into(), "y".into()).to_string(), "`xy");

    assert_eq!(Expr::l("x".into(), "y".into()).to_string(), "^x.y");

    assert_eq!(
        Expr::l("x".into(), Expr::a("y".into(), "z".into())).to_string(),
        "^x.`yz"
    );
    assert_eq!(
        Expr::l("X".into(), Expr::a("y".into(), "z".into())).to_string(),
        "^X.`yz"
    );

    assert_eq!(Expr::a("x".into(), "Y".into()).to_string(), "`xY");
    assert_eq!(Expr::a("X".into(), "y".into()).to_string(), "`Xy");
    assert_eq!(Expr::a("X".into(), "Y".into()).to_string(), "`X Y");
    assert_eq!(Expr::a("X".into(), ":Y".into()).to_string(), "`X:Y");
    assert_eq!(Expr::a(":X".into(), "Y".into()).to_string(), "`:X Y");
    assert_eq!(Expr::a(":X".into(), ":Y".into()).to_string(), "`:X:Y");
}

#[test]
fn test_tokens() {
    assert_eq!(
        tokens(&"x".into()),
        vec![Token::LowerIdent(Ident::Variable("x"))]
    );
    assert_eq!(
        tokens(&"FOO".into()),
        vec![Token::UpperIdent(Ident::Variable("FOO"))]
    );
    assert_eq!(
        tokens(&":a".into()),
        vec![Token::LowerIdent(Ident::Symbol("a"))]
    );
    assert_eq!(
        tokens(&":BAR".into()),
        vec![Token::UpperIdent(Ident::Symbol("BAR"))]
    );
    assert_eq!(
        tokens(&Expr::a("x".into(), "y".into())),
        vec![
            Token::LowerIdent(Ident::Variable("y")),
            Token::LowerIdent(Ident::Variable("x")),
            Token::Apply
        ]
    );
    assert_eq!(
        tokens(&Expr::l("x".into(), "y".into())),
        vec![
            Token::LowerIdent(Ident::Variable("y")),
            Token::Dot,
            Token::LowerIdent(Ident::Variable("x")),
            Token::Lambda
        ]
    );
}
