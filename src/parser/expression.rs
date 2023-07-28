use combine::parser::char::{char, spaces};
use combine::parser::choice::choice;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{parser, ParseError, Parser, Stream};

use crate::expression::Expr;
use crate::parser::identifier::identifier;

pub fn expr<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    expr_()
}

parser! {
    fn expr_[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces().with(choice((
            apply(),
            lambda(),
            symbol(),
            var(),
        )))
    }
}

#[test]
fn test_expr() {
    assert_eq!(expr().easy_parse("a"), Ok((Expr::v("a"), "")));
    assert_eq!(
        expr().easy_parse("`ab"),
        Ok((Expr::a("a".into(), "b".into()), ""))
    );
}

// ========================================================================== //

pub fn var<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    identifier().map(Expr::Variable)
}

#[test]
fn test_var() {
    assert!(var().parse(":abc").is_err());
    assert!(var().parse("^abc").is_err());

    assert_eq!(var().parse("abc"), Ok(("a".into(), "bc")));
    assert_eq!(var().parse("ABCabc"), Ok(("ABC".into(), "abc")));
}

// ========================================================================== //

pub fn symbol<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    char(':').with(identifier()).map(Expr::Symbol)
}

#[test]
fn test_symbol() {
    assert!(symbol().parse("abc").is_err());

    assert_eq!(symbol().parse(":abc"), Ok((":a".into(), "bc")));
    assert_eq!(symbol().parse(":ABCabc"), Ok((":ABC".into(), "abc")));
}

// ========================================================================== //

parser! {
    fn apply[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces()
            .with(char('`'))
            .with(
                expr()
                .and(expr())
            )
            .map(|(lhs, rhs)| Expr::a(lhs, rhs))
    }
}

#[test]
fn test_apply() {
    assert!(expr().easy_parse("`a").is_err());

    assert_eq!(
        expr().easy_parse("`ab"),
        Ok((Expr::a("a".into(), "b".into()), ""))
    );
    assert_eq!(
        expr().easy_parse(" ` a b"),
        Ok((Expr::a("a".into(), "b".into()), ""))
    );
    assert_eq!(
        expr().easy_parse("``abc"),
        Ok((Expr::a(Expr::a("a".into(), "b".into()), "c".into()), ""))
    );
    assert_eq!(
        expr().easy_parse(" ` ` a b c"),
        Ok((Expr::a(Expr::a("a".into(), "b".into()), "c".into()), ""))
    );
    assert_eq!(
        expr().easy_parse("`FOO BAR"),
        Ok((Expr::a("FOO".into(), "BAR".into()), ""))
    );
}

// ========================================================================== //

parser! {
    fn lambda[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces()
            .with(char('^'))
            .with(
                identifier()
                .skip(spaces().with(char('.'))
            )
            .and(expr()))
            .map(|(param, body)| Expr::l(param, body))
    }
}

#[test]
fn test_lambda() {
    assert!(expr().easy_parse("^a").is_err());

    assert_eq!(
        expr().easy_parse("^a.b"),
        Ok((Expr::l("a".into(), "b".into()), ""))
    );
    assert_eq!(
        expr().easy_parse(" ^ a . b"),
        Ok((Expr::l("a".into(), "b".into()), ""))
    );
    assert_eq!(
        expr().easy_parse("^a.^b.c"),
        Ok((Expr::l("a".into(), Expr::l("b".into(), "c".into())), ""))
    );
    assert_eq!(
        expr().easy_parse(" ^ a . ^ b . c"),
        Ok((Expr::l("a".into(), Expr::l("b".into(), "c".into())), ""))
    );
}
