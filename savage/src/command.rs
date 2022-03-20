// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021-2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use std::str::FromStr;

use chumsky::prelude::*;
use savage_core::{
    expression::Expression,
    parse::{parser as expression, Error},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Command {
    EvaluateExpression(Expression),
    DefineVariable(String, Expression),
    DefineFunction(String, Vec<String>, Expression),
    ShowHelp(Option<String>),
}

fn parser() -> impl Parser<char, Command, Error = Error> {
    text::ident()
        .padded()
        .then_ignore(just('='))
        .then(expression())
        .map(|(identifier, expression)| Command::DefineVariable(identifier, expression))
        .or(text::ident()
            .padded()
            .then(
                text::ident()
                    .padded()
                    .separated_by(just(','))
                    .padded()
                    .delimited_by(just('('), just(')'))
                    .padded(),
            )
            .then_ignore(just('='))
            .then(expression())
            .map(|((identifier, argument_identifiers), expression)| {
                Command::DefineFunction(identifier, argument_identifiers, expression)
            }))
        .or(expression().map(Command::EvaluateExpression))
        .or(just('?')
            .padded()
            .ignore_then(text::ident().padded().or_not())
            .map(Command::ShowHelp))
}

impl FromStr for Command {
    type Err = Vec<Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parser().then_ignore(end()).parse(string)
    }
}

#[cfg(test)]
mod tests {
    use savage_core::helpers::*;

    use crate::command::{Command, Command::*};

    #[track_caller]
    fn t(string: &str, command: Command) {
        assert_eq!(string.parse(), Ok(command));
    }

    #[test]
    fn parse() {
        t("   a ", EvaluateExpression(var("a")));
        t("a ==b ", EvaluateExpression(eq(var("a"), var("b"))));

        t(" a=1", DefineVariable("a".to_owned(), int(1)));
        t(
            "a   =b==  c",
            DefineVariable("a".to_owned(), eq(var("b"), var("c"))),
        );

        t("f(  )= 1", DefineFunction("f".to_owned(), vec![], int(1)));
        t(
            " f (x) =x ^ 2",
            DefineFunction("f".to_owned(), vec!["x".to_owned()], pow(var("x"), int(2))),
        );
        t(
            "f( x )=( (x)== (y))  ",
            DefineFunction("f".to_owned(), vec!["x".to_owned()], eq(var("x"), var("y"))),
        );
        t(
            "f( x ,y)= g(x, y)",
            DefineFunction(
                "f".to_owned(),
                vec!["x".to_owned(), "y".to_owned()],
                fun(var("g"), [var("x"), var("y")]),
            ),
        );

        t(" ?  ", ShowHelp(None));
        t("?is_prime  ", ShowHelp(Some("is_prime".to_owned())));
        t("?  is_prime", ShowHelp(Some("is_prime".to_owned())));
    }
}
