use combine::parser::char::{char, digit, lower, spaces, upper};
use combine::parser::choice::choice;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{many1, parser, ParseError, Parser, Stream};

use crate::expression::{Expr, Identifier};

fn identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(choice((short_identifier(), long_identifier())))
}

#[test]
fn test_identifier() {
    assert_eq!(identifier().parse("abc"), Ok(("a".into(), "bc")));
    assert_eq!(identifier().parse("ABC"), Ok(("ABC".into(), "")));
    assert_eq!(identifier().parse("ABCabc"), Ok(("ABC".into(), "abc")));
    assert_eq!(identifier().parse("A_B_C"), Ok(("A_B_C".into(), "")));
    assert_eq!(identifier().parse("42"), Ok(("42".into(), "")));

    assert!(identifier().parse(":abc").is_err());
    assert!(identifier().parse("^abc").is_err());
}

fn short_identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    lower().map(|c| Identifier::new(&c.to_string()))
}

#[test]
fn test_short_identifier() {
    assert_eq!(short_identifier().parse("a"), Ok(("a".into(), "")));

    assert!(short_identifier().parse("A").is_err());
}

fn long_identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(choice((digit(), upper(), char('_')))).map(|s: String| Identifier::new(&s))
}

#[test]
fn test_long_identifier() {
    assert!(long_identifier().parse("abc").is_err());

    assert_eq!(long_identifier().parse("ABC"), Ok(("ABC".into(), "")));
    assert_eq!(long_identifier().parse("ABCabc"), Ok(("ABC".into(), "abc")));
}

// ========================================================================== //

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
