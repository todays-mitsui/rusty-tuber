use std::fmt::Display;

use crate::expression::Expr;
use crate::identifier::Ident;
use regex::Regex;

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(&mut tokens(self)))
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    UpperIdent(String),
    LowerIdent(String),
    Apply,
    Lambda,
    Dot,
}

impl Display for Token {
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

fn tokens(expr: &Expr) -> Vec<Token> {
    match expr {
        Expr::Variable(i) => {
            let s = i.to_string();
            if is_upper_ident(&s) {
                vec![Token::UpperIdent(s)]
            } else {
                vec![Token::LowerIdent(s)]
            }
        }

        Expr::Symbol(i) => {
            let s = i.to_string();
            if is_upper_ident(&s) {
                vec![Token::UpperIdent(format!(":{}", i))]
            } else {
                vec![Token::LowerIdent(format!(":{}", i))]
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
            let s = param.to_string();
            if is_upper_ident(&s) {
                body.push(Token::UpperIdent(param.to_string()));
            } else {
                body.push(Token::LowerIdent(param.to_string()));
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
            0 => {}
            1 => {
                let t = tokens.pop().unwrap();
                str.push_str(&t.to_string())
            }
            _ => {
                let t1 = tokens.pop().unwrap();
                let t2 = tokens.pop().unwrap();
                match (t1, t2) {
                    (Token::UpperIdent(i1), Token::UpperIdent(i2)) => {
                        str.push_str(format!("{} ", i1).as_str());
                        tokens.push(Token::UpperIdent(i2));
                    }
                    (t1, t2) => {
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

    assert_eq!(Expr::a("x".into(), "Y".into()).to_string(), "`xY");
    assert_eq!(Expr::a("X".into(), "y".into()).to_string(), "`Xy");
    assert_eq!(Expr::a("X".into(), "Y".into()).to_string(), "`X Y");
}

#[test]
fn test_tokens() {
    assert_eq!(tokens(&"x".into()), vec![Token::LowerIdent("x".into())]);
    assert_eq!(tokens(&"FOO".into()), vec![Token::UpperIdent("FOO".into())]);
    assert_eq!(tokens(&":a".into()), vec![Token::LowerIdent(":a".into())]);
    assert_eq!(
        tokens(&":BAR".into()),
        vec![Token::UpperIdent(":BAR".into())]
    );
    assert_eq!(
        tokens(&Expr::a("x".into(), "y".into())),
        vec![
            Token::LowerIdent("y".into()),
            Token::LowerIdent("x".into()),
            Token::Apply
        ]
    );
    assert_eq!(
        tokens(&Expr::l("x".into(), "y".into())),
        vec![
            Token::LowerIdent("y".into()),
            Token::Dot,
            Token::LowerIdent("x".into()),
            Token::Lambda
        ]
    );
}
