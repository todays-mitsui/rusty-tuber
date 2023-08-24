use combine::parser::char::{char, digit, spaces, string};
use combine::parser::choice::choice;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{attempt, eof, many1, parser, ParseError, Parser, Stream};

use crate::command::Command;
use crate::expression::Expr;
use crate::function::Func;
use crate::identifier::Ident;
use crate::parser::expression::expr;
use crate::parser::identifier::identifier;

pub fn parse_command(s: &str) -> Result<Command, String> {
    command()
        .easy_parse(s)
        .map(|(c, _)| c)
        .map_err(|e| format!("{}", e))
}

// ========================================================================== //

fn command<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    choice((
        attempt(update()),
        eval(),
        attempt(eval_head()),
        attempt(eval_tail()),
        eval_last(),
        attempt(unlambda()),
        attempt(info()),
        global(),
    ))
    .skip(spaces())
    .skip(eof())
}

// ========================================================================== //

fn update<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    def_lhs()
        .skip(spaces().with(char('=')))
        .and(expr())
        .map(|((i, is), rhs)| match rhs {
            Expr::Variable(j) if is.is_empty() && i == j => return Command::Del(i),
            _ => Command::Update(Func::new(i, is, rhs)),
        })
}

fn def_lhs<Input>() -> impl Parser<Input, Output = (Ident, Vec<Ident>)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    def_lhs_()
}

parser! {
    fn def_lhs_[Input]()(Input) -> (Ident, Vec<Ident>)
    where [
        Input: Stream<Token = char>,
        Input::Error: ParseError<char, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        spaces().with(choice((
            char('`')
                .with(spaces())
                .with(def_lhs().and(identifier()))
                .map(|(mut lhs, i)| {
                    lhs.1.push(i);
                    lhs
                }),
            identifier().map(|i| (i, vec![])),
        )))
    }
}

// ========================================================================== //

fn eval<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    expr().map(Command::Eval)
}

fn eval_last<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    char('!').with(expr()).map(Command::EvalLast)
}

fn eval_head<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    let len = many1(digit()).and_then(|x: String| x.parse::<usize>());

    char('!')
        .with(len)
        .and(spaces().with(expr()))
        .map(|(len, e)| Command::EvalHead(len, e))
}

fn eval_tail<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    let len = many1(digit()).and_then(|x: String| x.parse::<usize>());

    string("!-")
        .with(len)
        .and(spaces().with(expr()))
        .map(|(len, e)| Command::EvalTail(len, e))
}

// ========================================================================== //

fn info<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces()
        .skip(char('?'))
        .with(identifier())
        .map(Command::Info)
}

fn global<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().skip(char('?')).map(|_| Command::Global)
}

// ========================================================================== //

fn unlambda<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    spaces()
        .skip(string("??"))
        .with(expr())
        .map(Command::Unlambda)
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::expression::Expr;

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("f=g"),
            Ok(Command::Update(Func::new("f".into(), vec![], "g".into())))
        );

        assert_eq!(
            parse_command("`ix = x"),
            Ok(Command::Update(Func::new(
                "i".into(),
                vec!["x".into()],
                "x".into()
            )))
        );

        assert_eq!(
            parse_command("```sxyz = ``xz`yz"),
            Ok(Command::Update(Func::new(
                "s".into(),
                vec!["x".into(), "y".into(), "z".into()],
                Expr::a(
                    Expr::a("x".into(), "z".into()),
                    Expr::a("y".into(), "z".into())
                )
            )))
        );

        assert_eq!(
            parse_command("`ab"),
            Ok(Command::Eval(Expr::a("a".into(), "b".into())))
        );

        assert_eq!(parse_command("? a"), Ok(Command::Info("a".into())));

        assert_eq!(parse_command("?"), Ok(Command::Global));

        assert!(parse_command("f=g h=i").is_err());
    }

    #[test]
    fn test_command() {
        assert_eq!(
            command().easy_parse("f=g"),
            Ok((
                Command::Update(Func::new("f".into(), vec![], "g".into())),
                ""
            ))
        );

        assert_eq!(
            command().easy_parse("`ix = x"),
            Ok((
                Command::Update(Func::new("i".into(), vec!["x".into()], "x".into())),
                ""
            ))
        );

        assert_eq!(
            command().easy_parse("```sxyz = ``xz`yz"),
            Ok((
                Command::Update(Func::new(
                    "s".into(),
                    vec!["x".into(), "y".into(), "z".into()],
                    Expr::a(
                        Expr::a("x".into(), "z".into()),
                        Expr::a("y".into(), "z".into())
                    )
                )),
                ""
            ))
        );

        assert_eq!(
            command().easy_parse("`ab"),
            Ok((Command::Eval(Expr::a("a".into(), "b".into())), ""))
        );

        assert_eq!(
            command().easy_parse("!`ab"),
            Ok((Command::EvalLast(Expr::a("a".into(), "b".into())), ""))
        );

        assert_eq!(
            command().easy_parse("!42 `ab"),
            Ok((Command::EvalHead(42, Expr::a("a".into(), "b".into())), ""))
        );

        assert_eq!(
            command().easy_parse("!-42 `ab"),
            Ok((Command::EvalTail(42, Expr::a("a".into(), "b".into())), ""))
        );

        assert_eq!(
            command().easy_parse("? a"),
            Ok((Command::Info("a".into()), ""))
        );

        assert_eq!(command().easy_parse("?"), Ok((Command::Global, "")));
    }

    #[test]
    fn test_def() {
        assert_eq!(
            update().easy_parse("f=g"),
            Ok((
                Command::Update(Func::new("f".into(), vec![], "g".into())),
                ""
            ))
        );

        assert_eq!(
            update().easy_parse("f = g"),
            Ok((
                Command::Update(Func::new("f".into(), vec![], "g".into())),
                ""
            ))
        );

        assert_eq!(
            update().easy_parse("`ix = x"),
            Ok((
                Command::Update(Func::new("i".into(), vec!["x".into()], "x".into())),
                ""
            ))
        );

        assert_eq!(
            update().easy_parse("```sxyz = ``xz`yz"),
            Ok((
                Command::Update(Func::new(
                    "s".into(),
                    vec!["x".into(), "y".into(), "z".into()],
                    Expr::a(
                        Expr::a("x".into(), "z".into()),
                        Expr::a("y".into(), "z".into())
                    )
                )),
                ""
            ))
        );
    }

    #[test]
    fn test_def_lhs() {
        assert_eq!(def_lhs().easy_parse("f"), Ok((("f".into(), vec![]), "")));

        assert_eq!(
            def_lhs().easy_parse("`fx"),
            Ok((("f".into(), vec!["x".into()]), ""))
        );

        assert_eq!(
            def_lhs().easy_parse("` f  x"),
            Ok((("f".into(), vec!["x".into()]), ""))
        );

        assert_eq!(
            def_lhs().easy_parse("``fxy"),
            Ok((("f".into(), vec!["x".into(), "y".into()]), ""))
        );

        assert_eq!(
            def_lhs().easy_parse("` `  f   x    y"),
            Ok((("f".into(), vec!["x".into(), "y".into()]), ""))
        );

        assert!(def_lhs().easy_parse("`f`xy").is_err());
    }

    #[test]
    fn test_eval() {
        assert_eq!(eval().easy_parse("a"), Ok((Command::Eval("a".into()), "")));
        assert_eq!(
            eval().easy_parse("`ab"),
            Ok((Command::Eval(Expr::a("a".into(), "b".into())), ""))
        );
    }

    #[test]
    fn test_info() {
        assert_eq!(info().easy_parse("?a"), Ok((Command::Info("a".into()), "")));
        assert_eq!(
            info().easy_parse("? a"),
            Ok((Command::Info("a".into()), ""))
        );
    }

    #[test]
    fn test_global() {
        assert_eq!(global().easy_parse("?"), Ok((Command::Global, "")));
    }

    #[test]
    fn test_unlambda() {
        assert_eq!(
            unlambda().easy_parse("??^x.x"),
            Ok((Command::Unlambda(Expr::l("x".into(), "x".into())), ""))
        );
    }
}
