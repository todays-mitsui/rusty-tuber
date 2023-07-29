use combine::parser::char::{char, spaces};
use combine::parser::choice::choice;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{eof, parser, ParseError, Parser, Stream};

use crate::command::Command;
use crate::function::Func;
use crate::identifier::Ident;
use crate::parser::expression::expr;
use crate::parser::identifier::identifier;

// pub fn command<Input>() -> impl Parser<Input, Output = Command>
// where
//     Input: Stream<Token = char>,
//     Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
//     <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
//         From<::std::num::ParseIntError>,
// {
//     choice((
//         update(),
//         eval(),
//         info(),
//         global(),
//     )).skip(spaces()).skip(eof())
// }

// ========================================================================== //

pub fn update<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    def_lhs()
        .skip(spaces().with(char('=')))
        .and(expr())
        .map(|((i, is), rhs)| Command::Update(i, Func::new(is, rhs)))
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

pub fn eval<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
        From<::std::num::ParseIntError>,
{
    expr().map(Command::Eval)
}

// ========================================================================== //

pub fn info<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces()
        .skip(char('?'))
        .with(identifier())
        .map(Command::Info)
}

pub fn global<Input>() -> impl Parser<Input, Output = Command>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().skip(char('?')).map(|_| Command::Global)
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::expression::Expr;

    // #[test]
    // fn test_command() {
    //     assert_eq!(
    //         command().easy_parse("f=g"),
    //         Ok((
    //             Command::Update("f".into(), Func::new(vec![], "g".into())),
    //             ""
    //         ))
    //     );

    //     assert_eq!(
    //         command().easy_parse("`ix = x"),
    //         Ok((
    //             Command::Update("i".into(), Func::new(vec!["x".into()], "x".into())),
    //             ""
    //         ))
    //     );

    //     assert_eq!(
    //         command().easy_parse("```sxyz = ``xz`yz"),
    //         Ok((
    //             Command::Update(
    //                 "s".into(),
    //                 Func::new(
    //                     vec!["x".into(), "y".into(), "z".into()],
    //                     Expr::a(
    //                         Expr::a("x".into(), "z".into()),
    //                         Expr::a("y".into(), "z".into())
    //                     )
    //                 )
    //             ),
    //             ""
    //         ))
    //     );

    //     assert_eq!(
    //         command().easy_parse("`ab"),
    //         Ok((Command::Eval(Expr::a("a".into(), "b".into())), ""))
    //     );

    //     assert_eq!(
    //         command().easy_parse("? a"),
    //         Ok((Command::Info("a".into()), ""))
    //     );

    //     assert_eq!(command().easy_parse("?"), Ok((Command::Global, "")));
    // }

    #[test]
    fn test_def() {
        assert_eq!(
            update().easy_parse("f=g"),
            Ok((
                Command::Update("f".into(), Func::new(vec![], "g".into())),
                ""
            ))
        );

        assert_eq!(
            update().easy_parse("f = g"),
            Ok((
                Command::Update("f".into(), Func::new(vec![], "g".into())),
                ""
            ))
        );

        assert_eq!(
            update().easy_parse("`ix = x"),
            Ok((
                Command::Update("i".into(), Func::new(vec!["x".into()], "x".into())),
                ""
            ))
        );

        assert_eq!(
            update().easy_parse("```sxyz = ``xz`yz"),
            Ok((
                Command::Update(
                    "s".into(),
                    Func::new(
                        vec!["x".into(), "y".into(), "z".into()],
                        Expr::a(
                            Expr::a("x".into(), "z".into()),
                            Expr::a("y".into(), "z".into())
                        )
                    )
                ),
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
}
