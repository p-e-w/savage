// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use std::cmp::max;
use std::fmt::{Display, Formatter, Result};

use num::{One, Signed, Zero};

use crate::expression::{Expression, Integer, Rational};

/// Returns a pair of integers `(n, m)` such that `x = n / 10^m`,
/// or `None` if no such integers exist.
fn decimal_representation(x: &Rational) -> Option<(Integer, usize)> {
    // https://en.wikipedia.org/wiki/Decimal_representation#Finite_decimal_representations
    let mut denominator = x.denom().clone();

    let [power_of_2, power_of_5] = [2, 5].map(|n| {
        let mut power = 0;

        while (denominator.clone() % Integer::from(n)).is_zero() {
            denominator /= n;
            power += 1;
        }

        power
    });

    if denominator.is_one() {
        let multiplier = if power_of_2 < power_of_5 {
            Integer::from(2).pow(power_of_5 - power_of_2)
        } else {
            Integer::from(5).pow(power_of_2 - power_of_5)
        };

        Some((x.numer() * multiplier, max(power_of_2, power_of_5) as usize))
    } else {
        None
    }
}

impl Expression {
    /// Formats the expression as an infix operator with the minimally necessary parentheses.
    fn fmt_infix(&self, f: &mut Formatter<'_>, symbol: &str, a: &Self, b: &Self) -> Result {
        use crate::expression::Associativity::*;

        let (self_precedence, self_associativity) = self.precedence_and_associativity();
        let (a_precedence, _) = a.precedence_and_associativity();
        let (b_precedence, _) = b.precedence_and_associativity();

        let a_needs_parentheses = (a_precedence < self_precedence)
            || ((a_precedence == self_precedence) && (self_associativity == RightAssociative));
        let b_needs_parentheses = (b_precedence < self_precedence)
            || ((b_precedence == self_precedence) && (self_associativity == LeftAssociative));

        write!(
            f,
            "{}{}{} {} {}{}{}",
            if a_needs_parentheses { "(" } else { "" },
            a,
            if a_needs_parentheses { ")" } else { "" },
            symbol,
            if b_needs_parentheses { "(" } else { "" },
            b,
            if b_needs_parentheses { ")" } else { "" },
        )
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use crate::expression::{Expression::*, RationalRepresentation::*};

        match self {
            Integer(n) => write!(f, "{}", n),
            Rational(x, representation) => {
                match representation {
                    Fraction => write!(f, "{}", x),
                    Decimal => {
                        if let Some((mantissa, separator_position)) = decimal_representation(x) {
                            let mut string = mantissa.abs().to_string();

                            if separator_position > 0 {
                                if separator_position > string.len() - 1 {
                                    // Left-pad the string with enough zeros to be able
                                    // to insert the decimal separator at the indicated position.
                                    string = format!(
                                        "{}{}",
                                        "0".repeat(separator_position - (string.len() - 1)),
                                        string,
                                    );
                                }

                                string.insert(string.len() - separator_position, '.');
                            }

                            write!(f, "{}{}", if x.is_negative() { "-" } else { "" }, string)
                        } else {
                            // Fall back to fraction representation.
                            write!(f, "{}", x)
                        }
                    }
                }
            }
            Complex(z, representation) => {
                if z.im.is_zero() {
                    write!(f, "{}", Rational(z.re.clone(), *representation))
                } else if z.re.is_zero() {
                    if z.im.abs().is_one() {
                        write!(f, "{}i", if z.im.is_negative() { "-" } else { "" })
                    } else {
                        write!(f, "{}*i", Rational(z.im.clone(), *representation))
                    }
                } else if z.re.is_negative() && z.im.is_positive() {
                    if z.im.is_one() {
                        write!(f, "i - {}", Rational(z.re.abs(), *representation))
                    } else {
                        write!(
                            f,
                            "{}*i - {}",
                            Rational(z.im.clone(), *representation),
                            Rational(z.re.abs(), *representation),
                        )
                    }
                } else if z.im.abs().is_one() {
                    write!(
                        f,
                        "{} {} i",
                        Rational(z.re.clone(), *representation),
                        if z.im.is_negative() { "-" } else { "+" },
                    )
                } else {
                    write!(
                        f,
                        "{} {} {}*i",
                        Rational(z.re.clone(), *representation),
                        if z.im.is_negative() { "-" } else { "+" },
                        Rational(z.im.abs(), *representation),
                    )
                }
            }
            Vector(v) => write!(f, "{}", v), // TODO
            Matrix(m) => write!(f, "{}", m), // TODO
            Sum(a, b) => self.fmt_infix(f, "+", a, b),
            Difference(a, b) => self.fmt_infix(f, "-", a, b),
            Product(a, b) => self.fmt_infix(f, "*", a, b),
            Quotient(a, b) => self.fmt_infix(f, "/", a, b),
            Remainder(a, b) => self.fmt_infix(f, "%", a, b),
            Power(a, b) => self.fmt_infix(f, "^", a, b),
            Equal(a, b) => self.fmt_infix(f, "==", a, b),
            NotEqual(a, b) => self.fmt_infix(f, "!=", a, b),
            LessThan(a, b) => self.fmt_infix(f, "<", a, b),
            LessThanOrEqual(a, b) => self.fmt_infix(f, "<=", a, b),
            GreaterThan(a, b) => self.fmt_infix(f, ">", a, b),
            GreaterThanOrEqual(a, b) => self.fmt_infix(f, ">=", a, b),
            AbsoluteValue(a) => write!(f, "|{}|", a),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::Expression;
    use crate::helpers::*;

    #[track_caller]
    fn t(expression: Expression, string: &str) {
        assert_eq!(expression.to_string(), string);
    }

    #[test]
    fn integers() {
        t(int(0), "0");
        t(int(1), "1");
        t(int(-1), "-1");
        t(int(1234567890), "1234567890");
        t(int(-1234567890), "-1234567890");
        t(int(9876543210u64), "9876543210");
        t(int(-9876543210i64), "-9876543210");
    }

    #[test]
    fn rational_numbers() {
        t(rat(0, 1), "0");
        t(ratd(0, 1), "0");
        t(rat(0, -1), "0");
        t(ratd(0, -1), "0");
        t(rat(1, 1), "1");
        t(ratd(1, 1), "1");
        t(rat(-1, 1), "-1");
        t(ratd(-1, 1), "-1");
        t(rat(1, 2), "1/2");
        t(ratd(1, 2), "0.5");
        t(rat(3, 2), "3/2");
        t(ratd(3, 2), "1.5");
        t(rat(1, 3), "1/3");
        t(ratd(1, 3), "1/3");
        t(rat(123, 40), "123/40");
        t(ratd(123, 40), "3.075");
        t(rat(123, -40), "-123/40");
        t(ratd(123, -40), "-3.075");
        t(rat(-123, -40), "123/40");
        t(ratd(-123, -40), "3.075");
    }

    #[test]
    fn complex_numbers() {
        t(com(0, 1, 0, 1), "0");
        t(comd(0, 1, 0, 1), "0");
        t(com(1, 1, 0, 1), "1");
        t(comd(1, 1, 0, 1), "1");
        t(com(0, 1, 1, 1), "i");
        t(comd(0, 1, 1, 1), "i");
        t(com(-1, 1, 0, 1), "-1");
        t(comd(-1, 1, 0, 1), "-1");
        t(com(0, 1, -1, 1), "-i");
        t(comd(0, 1, -1, 1), "-i");
        t(com(1, 1, 1, 1), "1 + i");
        t(comd(1, 1, 1, 1), "1 + i");
        t(com(1, 1, -1, 1), "1 - i");
        t(comd(1, 1, -1, 1), "1 - i");
        t(com(-1, 1, 1, 1), "i - 1");
        t(comd(-1, 1, 1, 1), "i - 1");
        t(com(-1, 1, -1, 1), "-1 - i");
        t(comd(-1, 1, -1, 1), "-1 - i");
        t(com(123, -40, 1, 3), "1/3*i - 123/40");
        t(comd(123, -40, 1, 3), "1/3*i - 3.075");
        t(com(1, 3, 123, 40), "1/3 + 123/40*i");
        t(comd(1, 3, 123, 40), "1/3 + 3.075*i");
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
    fn operators() {
        t((int(1) + int(2)) + int(3), "1 + 2 + 3");
        t((int(1) + int(2)) - int(3), "1 + 2 - 3");
        t((int(1) - int(2)) + int(3), "1 - 2 + 3");
        t((int(1) - int(2)) - int(3), "1 - 2 - 3");
        t(int(1) + (int(2) + int(3)), "1 + 2 + 3");
        t(int(1) + (int(2) - int(3)), "1 + 2 - 3");
        t(int(1) - (int(2) + int(3)), "1 - (2 + 3)");
        t(int(1) - (int(2) - int(3)), "1 - (2 - 3)");

        t((int(1) * int(2)) * int(3), "1 * 2 * 3");
        t((int(1) * int(2)) / int(3), "1 * 2 / 3");
        t((int(1) / int(2)) * int(3), "1 / 2 * 3");
        t((int(1) / int(2)) / int(3), "1 / 2 / 3");
        t(int(1) * (int(2) * int(3)), "1 * 2 * 3");
        t(int(1) * (int(2) / int(3)), "1 * 2 / 3");
        t(int(1) / (int(2) * int(3)), "1 / (2 * 3)");
        t(int(1) / (int(2) / int(3)), "1 / (2 / 3)");

        t((int(1) + int(2)) / int(3), "(1 + 2) / 3");
        t((int(1) / int(2)) + int(3), "1 / 2 + 3");
        t(int(1) + (int(2) / int(3)), "1 + 2 / 3");
        t(int(1) / (int(2) + int(3)), "1 / (2 + 3)");

        t(pow(int(1) * int(2), int(3)), "(1 * 2) ^ 3");
        t(pow(int(1), int(2)) * int(3), "1 ^ 2 * 3");
        t(int(1) * pow(int(2), int(3)), "1 * 2 ^ 3");
        t(pow(int(1), int(2) * int(3)), "1 ^ (2 * 3)");

        t(pow(pow(int(1), int(2)), int(3)), "(1 ^ 2) ^ 3");
        t(pow(int(1), pow(int(2), int(3))), "1 ^ 2 ^ 3");

        t(pow(int(1), int(2)), "1 ^ 2");
        t(pow(int(-1), int(2)), "(-1) ^ 2");
        t(pow(rat(1, 2), int(3)), "(1/2) ^ 3");
        t(pow(ratd(1, 2), int(3)), "0.5 ^ 3");
        t(pow(com(0, 1, -1, 1), int(2)), "(-i) ^ 2");
        t(com(1, 1, 1, 1) * int(2), "(1 + i) * 2");
        t(com(1, 1, -1, 1) - int(2), "1 - i - 2");
        t(int(2) - com(1, 1, -1, 1), "2 - (1 - i)");
    }
}
