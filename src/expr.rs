pub mod free_vars;
pub mod parser;

use std::collections::HashSet;

use crate::env::Env;
use crate::expr::free_vars::FreeVars;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(s: &str) -> Identifier {
        Identifier(String::from(s))
    }

    pub fn new_name(&self, vars: &HashSet<Identifier>) -> Identifier {
        let mut name = self.0.to_uppercase();

        if !vars.contains(&Identifier(name.clone())) {
            return Identifier(name);
        }

        let mut i = 0;
        while vars.contains(&Identifier(name.clone())) {
            name = format!("{}{}", self.0.to_uppercase(), i);
            i += 1;
        }
        Identifier(name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Variable(Identifier),
    Symbol(Identifier),
    Apply { lhs: Box<Expr>, rhs: Box<Expr> },
    Lambda { param: Identifier, body: Box<Expr> },
}

impl Expr {
    pub fn destruct_apply(self) -> (Expr, Expr) {
        match self {
            Expr::Apply { lhs, rhs } => (*lhs, *rhs),
            _ => panic!("destruct_apply: not an apply"),
        }
    }

    pub fn v(label: &str) -> Expr {
        Expr::Variable(Identifier(String::from(label)))
    }

    pub fn s(label: &str) -> Expr {
        Expr::Symbol(Identifier(String::from(label)))
    }

    pub fn a(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Apply {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn l(param: Identifier, body: Expr) -> Expr {
        Expr::Lambda {
            param,
            body: Box::new(body),
        }
    }

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
                if param == old {
                    *param = new.clone();
                }
                body.rename_var(old, new);
            }
        }
    }

    pub fn arity(&self, env: &Env) -> Option<usize> {
        match self {
            Expr::Lambda { .. } => Some(1),
            Expr::Variable(id) => env.arity(id),
            _ => None,
        }
    }
}

impl Expr {
    pub fn apply(&self, env: &Env, args: Vec<Expr>) -> Expr {
        match self {
            Expr::Lambda { param, body } => {
                body.clone().substitute(&param, &args[0])
            }

            Expr::Variable(id) => {
                match env.get(&id) {
                    Some(func) => func.apply(args),
                    None => panic!("apply: not found"),
                }
            }

            _ => panic!("apply: not a function"),
        }
    }
}

pub fn eval(lhs: &Expr, rhs: &Expr) -> Option<Expr> {
    match lhs {
        Expr::Lambda { param, body } => Some(body.clone().substitute(param, rhs)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_i() {
        let i = Expr::l(Identifier::new("x"), Expr::v("x"));
        let a = Expr::s("a");
        let expected = Expr::s("a");
        assert_eq!(eval(&i, &a), Some(expected));
    }

    #[test]
    fn test_eval_k() {
        let k = Expr::l(
            Identifier::new("x"),
            Expr::l(Identifier::new("y"), Expr::v("x")),
        );

        assert_eq!(
            eval(&k, &k),
            Some(Expr::l(
                Identifier::new("y"),
                Expr::l(
                    Identifier::new("x"),
                    Expr::l(Identifier::new("y"), Expr::v("x")),
                ),
            ))
        );
        assert_eq!(
            eval(&eval(&k, &k).unwrap(), &Expr::s("a")),
            Some(Expr::l(
                Identifier::new("x"),
                Expr::l(Identifier::new("y"), Expr::v("x")),
            ))
        );
    }

    #[test]
    fn test_eval_s() {
        let s = Expr::l(
            Identifier::new("x"),
            Expr::l(
                Identifier::new("y"),
                Expr::l(
                    Identifier::new("z"),
                    Expr::a(
                        Expr::a(Expr::v("x"), Expr::v("z")),
                        Expr::a(Expr::v("y"), Expr::v("z")),
                    ),
                ),
            ),
        );
        let y = Expr::v("y");

        assert_eq!(
            eval(&s, &y),
            Some(Expr::l(
                Identifier::new("Y"),
                Expr::l(
                    Identifier::new("z"),
                    Expr::a(
                        Expr::a(Expr::v("y"), Expr::v("z")),
                        Expr::a(Expr::v("Y"), Expr::v("z")),
                    ),
                ),
            ))
        );
    }

    #[test]
    fn test_eval_other() {
        let s = Expr::s("s");
        let v = Expr::v("v");
        let a = Expr::a(Expr::v("x"), Expr::v("y"));
        assert_eq!(eval(&v, &s), None);
        assert_eq!(eval(&s, &s), None);
        assert_eq!(eval(&a, &s), None);
    }

    #[test]
    fn test_expr_substitute() {
        // ^z.x [x := y] => ^z.y
        assert_eq!(
            Expr::l(Identifier::new("z"), Expr::v("x"))
                .substitute(&Identifier::new("x"), &Expr::v("y")),
            Expr::l(Identifier::new("z"), Expr::v("y"))
        );

        // ^Y.^y.`xY [x := y] => ^Y.^Y0.`yY
        assert_eq!(
            Expr::l(
                Identifier::new("Y"),
                Expr::l(Identifier::new("y"), Expr::a(Expr::v("x"), Expr::v("Y")))
            )
            .substitute(&Identifier::new("x"), &Expr::v("y")),
            Expr::l(
                Identifier::new("Y"),
                Expr::l(Identifier::new("Y0"), Expr::a(Expr::v("y"), Expr::v("Y")))
            )
        );
    }
}
