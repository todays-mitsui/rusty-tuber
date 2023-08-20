use crate::function::Func;
use regex::Regex;
use std::fmt::Display;

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tokens = tokens(self);
        write!(
            f,
            "{}{} = {}",
            "`".to_string().repeat(self.arity()),
            to_string(&mut tokens),
            self.body()
        )
    }
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    UpperIdent(&'a str),
    LowerIdent(&'a str),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::UpperIdent(i) => write!(f, "{}", i),
            Token::LowerIdent(i) => write!(f, "{}", i),
        }
    }
}

fn tokens<'a>(f: &'a Func) -> Vec<Token<'a>> {
    let mut tokens = f
        .params
        .iter()
        .map(|i| {
            let label = i.label();
            if is_upper_ident(label) {
                Token::UpperIdent(label)
            } else {
                Token::LowerIdent(label)
            }
        })
        .collect::<Vec<_>>();

    let label = f.name.label();
    if is_upper_ident(label) {
        tokens.insert(0, Token::UpperIdent(label));
    } else {
        tokens.insert(0, Token::LowerIdent(label));
    }

    tokens
}

fn to_string(tokens: &mut Vec<Token>) -> String {
    tokens.reverse();
    let mut str = String::new();
    while tokens.len() > 0 {
        match tokens.len() {
            1 => {
                let t = tokens.pop().unwrap();
                str.push_str(&format!("{}", t));
            }

            _ => {
                let t1 = tokens.pop().unwrap();
                let t2 = tokens.pop().unwrap();

                match (t1, &t2) {
                    (Token::UpperIdent(ident1), Token::UpperIdent(_)) => {
                        str.push_str(&format!("{} ", ident1));
                    }
                    (t1, _) => {
                        str.push_str(&format!("{}", t1));
                    }
                }

                tokens.push(t2);
            }
        }
    }
    str
}

fn is_upper_ident(s: &str) -> bool {
    let regex_upper_ident: Regex = Regex::new(r"^[A-Z_]+$").unwrap();
    regex_upper_ident.is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;

    #[test]
    fn test_to_string_1() {
        let f = Func {
            name: "f".into(),
            params: vec!["x".into(), "y".into()],
            body: Expr::a("x".into(), "y".into()),
        };
        assert_eq!(f.to_string(), "``fxy = `xy");
    }

    #[test]
    fn test_to_string_2() {
        let f = Func {
            name: "F".into(),
            params: vec!["X".into(), "Y".into()],
            body: Expr::a("X".into(), "Y".into()),
        };
        assert_eq!(f.to_string(), "``F X Y = `X Y");
    }

    #[test]
    fn test_to_string_3() {
        let f = Func {
            name: "F".into(),
            params: vec!["x".into(), "Y".into()],
            body: Expr::a("x".into(), "Y".into()),
        };
        assert_eq!(f.to_string(), "``FxY = `xY");
    }
}
