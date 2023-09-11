use combine::parser::char::{char, spaces, string};
use combine::parser::choice::choice;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{attempt, many, many1, optional, parser, ParseError, Parser, Stream};

use crate::expression::Expr;
use crate::identifier::Ident;
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
            attempt(apply()),
            attempt(lambda()),
            symbol(),
            var(),
        )))
    }
}

#[test]
fn test_expr() {
    assert_eq!(expr().easy_parse("a"), Ok((Expr::v("a"), "")));
    assert_eq!(
        expr().easy_parse("a(b)"),
        Ok((Expr::a("a".into(), "b".into()), ""))
    );
}

// ========================================================================== //

fn var<Input>() -> impl Parser<Input, Output = Expr>
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

fn symbol<Input>() -> impl Parser<Input, Output = Expr>
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
            .with(callable())
            .and(
                many1(spaces().with(args()))
            )
            .map(|(mut e, argss)| {
                let _: Vec<Vec<Expr>> = argss;
                for args in argss {
                    for arg in args {
                        e = Expr::a(e, arg);
                    }
                }
                e
            })
    }
}

fn callable<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    callable_()
}

parser! {
    fn callable_[Input]()(Input) -> Expr
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces().with(choice((
            parens(callable()),  // パーレンで囲まれている式はパーレンを剥がしてから再度パースを試みる
            attempt(lambda()),
            symbol(),
            var(),
        )))
    }
}

fn args<Input>() -> impl Parser<Input, Output = Vec<Expr>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    parens(
        optional(many(attempt(
            spaces().with(expr()).skip(spaces()).skip(char(',')),
        )))
        .and(spaces().with(expr())),
    )
    .map(|(es, e)| {
        let mut es = es.unwrap_or_else(Vec::new);
        es.push(e);
        es
    })
}

#[test]
fn test_apply() {
    assert_eq!(
        apply().easy_parse("a(b)"),
        Ok((Expr::a("a".into(), "b".into()), ""))
    );
    assert_eq!(
        apply().easy_parse("(a)(b)"),
        Ok((Expr::a("a".into(), "b".into()), ""))
    );
    assert_eq!(
        apply().easy_parse("a(b)(c)"),
        Ok((Expr::a(Expr::a("a".into(), "b".into()), "c".into()), ""))
    );
    assert_eq!(
        apply().easy_parse(" a (  b   )"),
        Ok((Expr::a("a".into(), "b".into()), ""))
    );
    assert_eq!(
        apply().easy_parse("a(b, c)"),
        Ok((Expr::a(Expr::a("a".into(), "b".into()), "c".into()), ""))
    );
    assert_eq!(
        apply().easy_parse(" a ( b ,  c  )"),
        Ok((Expr::a(Expr::a("a".into(), "b".into()), "c".into()), ""))
    );
    assert_eq!(
        apply().easy_parse("FOO(BAR)"),
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
            .with(
                choice((
                    params(),
                    identifier().map(|i| vec![i])),
                ))
                .skip(spaces().with(string("=>"))
            )
            .and(expr())
            .map(|(params, mut body)| {
                for param in params.into_iter().rev() {
                    body = Expr::l(param, body);
                }
                body
            })
    }
}

fn params<Input>() -> impl Parser<Input, Output = Vec<Ident>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    parens(
        optional(many(attempt(
            spaces().with(identifier()).skip(spaces()).skip(char(',')),
        )))
        .and(spaces().with(identifier())),
    )
    .map(|(is, i)| {
        let mut is = is.unwrap_or_else(Vec::new);
        is.push(i);
        is
    })
}

#[test]
fn test_lambda() {
    assert_eq!(
        lambda().easy_parse("a=>b"),
        Ok((Expr::l("a".into(), "b".into()), ""))
    );
    assert_eq!(
        lambda().easy_parse(" a   =>  b"),
        Ok((Expr::l("a".into(), "b".into()), ""))
    );
    assert_eq!(
        lambda().easy_parse("a => b => c"),
        Ok((Expr::l("a".into(), Expr::l("b".into(), "c".into())), ""))
    );
    assert_eq!(
        lambda().easy_parse("(a, b) => c"),
        Ok((Expr::l("a".into(), Expr::l("b".into(), "c".into())), ""))
    );
}

// ========================================================================== //

fn parens<Input, Output>(
    parser: impl Parser<Input, Output = Output>,
) -> impl Parser<Input, Output = Output>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    spaces()
        .with(char('('))
        .with(spaces().with(parser))
        .skip(spaces().with(char(')')))
}

#[test]
fn test_parens() {
    assert_eq!(parens(char('a')).easy_parse("(a)"), Ok(('a', "")));
    assert_eq!(parens(char('a')).easy_parse(" ( a )"), Ok(('a', "")));

    assert!(parens(char('a')).easy_parse("a").is_err());
    assert!(parens(char('a')).easy_parse("((a))").is_err());
}
