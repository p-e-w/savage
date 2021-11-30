// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use std::str::FromStr;

use chumsky::prelude::*;

use crate::{
    expression::{Expression, Integer},
    helpers::*,
};

#[allow(clippy::let_and_return)]
fn parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    recursive(|expression| {
        let identifier = text::ident()
            .map(|identifier: String| match identifier.as_str() {
                "true" => Expression::Boolean(true),
                "false" => Expression::Boolean(false),
                _ => var(identifier),
            })
            .labelled("identifier");

        let number = text::int(10)
            .chain(just('.').ignore_then(text::digits(10)).or_not())
            .map(|parts: Vec<String>| match parts.as_slice() {
                [integer] => int(integer.parse::<Integer>().unwrap()),
                [integer_part, fractional_part] => {
                    let numerator = format!("{}{}", integer_part, fractional_part);
                    let denominator = format!("1{}", "0".repeat(fractional_part.len()));
                    ratd(
                        numerator.parse::<Integer>().unwrap(),
                        denominator.parse::<Integer>().unwrap(),
                    )
                }
                _ => unreachable!(),
            })
            .labelled("number");

        let atomic_expression = identifier
            .or(number)
            .or(expression.clone().delimited_by('(', ')'))
            .padded();

        let function = atomic_expression
            .clone()
            .then(
                expression
                    .clone()
                    .separated_by(just(','))
                    .padded()
                    .delimited_by('(', ')'),
            )
            .map(|(function, arguments)| fun(function, arguments))
            .labelled("function")
            .or(atomic_expression)
            .padded();

        let power = function
            .clone()
            .then_ignore(just('^'))
            .repeated()
            .then(function)
            .foldr(pow)
            .labelled("power");

        let negation = just('-')
            .ignore_then(power.clone())
            .map(|a| -a)
            .or(just('!').ignore_then(power.clone()).map(|a| !a))
            .labelled("negation")
            .or(power)
            .padded();

        let product_or_quotient_or_remainder = negation
            .clone()
            .then(
                just('*')
                    .or(just('/'))
                    .or(just('%'))
                    .then(negation)
                    .repeated(),
            )
            .foldl(|a, (operator, b)| match operator {
                '*' => a * b,
                '/' => a / b,
                '%' => a % b,
                _ => unreachable!(),
            })
            .labelled("product_or_quotient_or_remainder");

        let sum_or_difference = product_or_quotient_or_remainder
            .clone()
            .then(
                just('+')
                    .or(just('-'))
                    .then(product_or_quotient_or_remainder)
                    .repeated(),
            )
            .foldl(|a, (operator, b)| match operator {
                '+' => a + b,
                '-' => a - b,
                _ => unreachable!(),
            })
            .labelled("sum_or_difference");

        let comparison = sum_or_difference
            .clone()
            .then(
                just('=')
                    .chain(just('='))
                    .or(just('!').chain(just('=')))
                    .or(just('<').chain(just('=')))
                    .or(just('<').to(vec!['<']))
                    .or(just('>').chain(just('=')))
                    .or(just('>').to(vec!['>']))
                    .collect::<String>()
                    .then(sum_or_difference)
                    .repeated(),
            )
            .foldl(|a, (operator, b)| match operator.as_str() {
                "==" => eq(a, b),
                "!=" => ne(a, b),
                "<" => lt(a, b),
                "<=" => le(a, b),
                ">" => gt(a, b),
                ">=" => ge(a, b),
                _ => unreachable!(),
            })
            .labelled("comparison");

        let conjunction = comparison
            .clone()
            .then(
                just('&')
                    .ignore_then(just('&'))
                    .ignore_then(comparison)
                    .repeated(),
            )
            .foldl(and)
            .labelled("conjunction");

        let disjunction = conjunction
            .clone()
            .then(
                just('|')
                    .ignore_then(just('|'))
                    .ignore_then(conjunction)
                    .repeated(),
            )
            .foldl(or)
            .labelled("disjunction");

        disjunction
    })
    .then_ignore(end())
}

impl FromStr for Expression {
    type Err = Vec<Simple<char>>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parser().parse(string)
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{Expression, Expression::*};
    use crate::helpers::*;

    #[track_caller]
    fn t(string: &str, expression: Expression) {
        assert_eq!(string.parse(), Ok(expression));
    }

    #[test]
    fn variables() {
        t("a   ", var("a"));
        t("     A", var("A"));
        t("  Named_Variable ", var("Named_Variable"));
    }

    #[test]
    fn functions() {
        t(" f(   )   ", fun(var("f"), []));
        t("f ( a) ", fun(var("f"), [var("a")]));
        t("f( a, 1  )", fun(var("f"), [var("a"), int(1)]));
        t(
            " f(  g(a ),h(   b )  ) ",
            fun(
                var("f"),
                [fun(var("g"), [var("a")]), fun(var("h"), [var("b")])],
            ),
        );
        t(
            " ( f ( a ) ) ( b ) ",
            fun(fun(var("f"), [var("a")]), [var("b")]),
        );
        t("(f +g)( a)", fun(var("f") + var("g"), [var("a")]));
    }

    #[test]
    fn integers() {
        t("0", int(0));
        t("  1", int(1));
        t("1234567890  ", int(1234567890));
        t("  9876543210  ", int(9876543210u64));
    }

    #[test]
    fn rational_numbers() {
        t("0.5", ratd(1, 2));
        t(" 1.5", ratd(3, 2));
        t("3.075 ", ratd(123, 40));
        t(" 100.000 ", ratd(100, 1));
    }

    #[test]
    fn vectors() {
        // TODO
    }

    #[test]
    fn matrices() {
        // TODO
    }

    #[test]
    fn booleans() {
        t("   true", Boolean(true));
        t("false   ", Boolean(false));
    }

    #[test]
    fn operators() {
        t("  - 1 ", -int(1));
        t(" - (-    1 )", -(-int(1)));
        t("!A  ", !var("A"));
        t(" !( ! A)", !(!var("A")));

        t("1+2    +3   ", (int(1) + int(2)) + int(3));
        t(" 1 +2- 3 ", (int(1) + int(2)) - int(3));
        t("1  -2 +    3", (int(1) - int(2)) + int(3));
        t("1-2-3", (int(1) - int(2)) - int(3));
        t(" 1-(2 + 3)", int(1) - (int(2) + int(3)));
        t("1 - (2 - 3)", int(1) - (int(2) - int(3)));

        t(" 1 *2* 3 ", (int(1) * int(2)) * int(3));
        t("1*2 / 3", (int(1) * int(2)) / int(3));
        t("1 / 2*3  ", (int(1) / int(2)) * int(3));
        t("  1/2/3", (int(1) / int(2)) / int(3));
        t("1 /(  2 *3) ", int(1) / (int(2) * int(3)));
        t(" 1/ (2 /3)", int(1) / (int(2) / int(3)));

        t("(1+2)   /3 ", (int(1) + int(2)) / int(3));
        t("  1/ 2 +3", (int(1) / int(2)) + int(3));
        t("1 + 2 / 3", int(1) + (int(2) / int(3)));
        t("1/ (2+3)     ", int(1) / (int(2) + int(3)));

        t("(1 * 2)^3", pow(int(1) * int(2), int(3)));
        t("1^2 * 3 ", pow(int(1), int(2)) * int(3));
        t("1*  2  ^3", int(1) * pow(int(2), int(3)));
        t(" 1^( 2 * 3 )", pow(int(1), int(2) * int(3)));

        t(" (1^2)  ^  3", pow(pow(int(1), int(2)), int(3)));
        t("1 ^2 ^3 ", pow(int(1), pow(int(2), int(3))));

        // TODO: Comparison operators!

        t("A&&B&&C", and(and(var("A"), var("B")), var("C")));
        t("A  &&  B||C", or(and(var("A"), var("B")), var("C")));
        t(" ( A || B ) && C ", and(or(var("A"), var("B")), var("C")));
        t("A||B ||C", or(or(var("A"), var("B")), var("C")));
        t("A&& ( B|| C)", and(var("A"), or(var("B"), var("C"))));
        t("   A|| B &&C", or(var("A"), and(var("B"), var("C"))));
    }
}
