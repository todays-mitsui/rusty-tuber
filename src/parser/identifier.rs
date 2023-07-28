use combine::parser::char::{char, digit, lower, spaces, upper};
use combine::parser::choice::choice;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{many1, ParseError, Parser, Stream};

use crate::identifier::Ident;

pub fn identifier<Input>() -> impl Parser<Input, Output = Ident>
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

fn short_identifier<Input>() -> impl Parser<Input, Output = Ident>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    lower().map(|c| Ident::new(&c.to_string()))
}

#[test]
fn test_short_identifier() {
    assert_eq!(short_identifier().parse("a"), Ok(("a".into(), "")));

    assert!(short_identifier().parse("A").is_err());
}

fn long_identifier<Input>() -> impl Parser<Input, Output = Ident>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(choice((digit(), upper(), char('_')))).map(|s: String| Ident::new(&s))
}

#[test]
fn test_long_identifier() {
    assert!(long_identifier().parse("abc").is_err());

    assert_eq!(long_identifier().parse("ABC"), Ok(("ABC".into(), "")));
    assert_eq!(long_identifier().parse("ABCabc"), Ok(("ABC".into(), "abc")));
}
