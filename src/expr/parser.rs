use super::{Expr, Identifier};
use combine::parser::char::{char, digit, lower, spaces, upper};
use combine::parser::choice::choice;
use combine::EasyParser;
use combine::{many1, ParseError, Parser, Stream};

fn identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(choice((short_identifier(), long_identifier())))
}

#[test]
fn test_identifier() {
    assert_eq!(
        identifier().parse("abc"),
        Ok((Identifier("a".to_string()), "bc"))
    );
    assert_eq!(
        identifier().parse("ABC"),
        Ok((Identifier("ABC".to_string()), ""))
    );
    assert_eq!(
        identifier().parse("ABCabc"),
        Ok((Identifier("ABC".to_string()), "abc"))
    );
    assert_eq!(
        identifier().parse("A_B_C"),
        Ok((Identifier("A_B_C".to_string()), ""))
    );
    assert_eq!(
        identifier().parse("42"),
        Ok((Identifier("42".to_string()), ""))
    );

    assert!(identifier().parse(":abc").is_err());
    assert!(identifier().parse("^abc").is_err());
}

fn short_identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    lower().map(|c| Identifier(c.to_string()))
}

#[test]
fn test_short_identifier() {
    assert_eq!(
        short_identifier().parse("a"),
        Ok((Identifier("a".to_string()), ""))
    );

    assert!(short_identifier().parse("A").is_err());
}

fn long_identifier<Input>() -> impl Parser<Input, Output = Identifier>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(choice((digit(), upper(), char('_')))).map(Identifier)
}

#[test]
fn test_long_identifier() {
    assert!(long_identifier().parse("abc").is_err());

    assert_eq!(
        long_identifier().parse("ABC"),
        Ok((Identifier("ABC".to_string()), ""))
    );
    assert_eq!(
        long_identifier().parse("ABCabc"),
        Ok((Identifier("ABC".to_string()), "abc"))
    );
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
    assert_eq!(expr().easy_parse("a"), Ok((Expr::var("a"), "")));
    assert_eq!(
        expr().easy_parse("`ab"),
        Ok((Expr::apply(Expr::var("a"), Expr::var("b")), ""))
    );
}

// ========================================================================== //

pub fn var<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    identifier().map(Expr::Var)
}

#[test]
fn test_var() {
    assert!(var().parse(":abc").is_err());
    assert!(var().parse("^abc").is_err());

    assert_eq!(var().parse("abc"), Ok((Expr::var("a"), "bc")));
    assert_eq!(var().parse("ABCabc"), Ok((Expr::var("ABC"), "abc")));
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

    assert_eq!(symbol().parse(":abc"), Ok((Expr::symbol("a"), "bc")));
    assert_eq!(symbol().parse(":ABCabc"), Ok((Expr::symbol("ABC"), "abc")));
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
            .map(|(lhs, rhs)| Expr::apply(lhs, rhs))
    }
}

#[test]
fn test_apply() {
    assert!(expr().easy_parse("`a").is_err());

    assert_eq!(
        expr().easy_parse("`ab"),
        Ok((Expr::apply(Expr::var("a"), Expr::var("b")), ""))
    );
    assert_eq!(
        expr().easy_parse(" ` a b"),
        Ok((Expr::apply(Expr::var("a"), Expr::var("b")), ""))
    );
    assert_eq!(
        expr().easy_parse("``abc"),
        Ok((
            Expr::apply(Expr::apply(Expr::var("a"), Expr::var("b")), Expr::var("c")),
            ""
        ))
    );
    assert_eq!(
        expr().easy_parse(" ` ` a b c"),
        Ok((
            Expr::apply(Expr::apply(Expr::var("a"), Expr::var("b")), Expr::var("c")),
            ""
        ))
    );
    assert_eq!(
        expr().easy_parse("`FOO BAR"),
        Ok((Expr::apply(Expr::var("FOO"), Expr::var("BAR")), ""))
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
            .map(|(param, body)| Expr::lambda(param, body))
    }
}

#[test]
fn test_lambda() {
    assert!(expr().easy_parse("^a").is_err());

    assert_eq!(
        expr().easy_parse("^a.b"),
        Ok((
            Expr::lambda(Identifier("a".to_string()), Expr::var("b")),
            ""
        ))
    );
    assert_eq!(
        expr().easy_parse(" ^ a . b"),
        Ok((
            Expr::lambda(Identifier("a".to_string()), Expr::var("b")),
            ""
        ))
    );
    assert_eq!(
        expr().easy_parse("^a.^b.c"),
        Ok((
            Expr::lambda(
                Identifier("a".to_string()),
                Expr::lambda(Identifier("b".to_string()), Expr::var("c"))
            ),
            ""
        ))
    );
    assert_eq!(
        expr().easy_parse(" ^ a . ^ b . c"),
        Ok((
            Expr::lambda(
                Identifier("a".to_string()),
                Expr::lambda(Identifier("b".to_string()), Expr::var("c"))
            ),
            ""
        ))
    );
}
