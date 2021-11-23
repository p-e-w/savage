// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

//! Operators, conversions, and helper functions to make working with expressions easier.

use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};

use crate::expression::{
    Complex, Expression, Integer, Matrix, Rational, RationalRepresentation, Vector,
};

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self {
        Expression::Negation(Box::new(self))
    }
}

impl Not for Expression {
    type Output = Self;

    fn not(self) -> Self {
        Expression::Not(Box::new(self))
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Expression::Sum(Box::new(self), Box::new(other))
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Expression::Difference(Box::new(self), Box::new(other))
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Expression::Product(Box::new(self), Box::new(other))
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Expression::Quotient(Box::new(self), Box::new(other))
    }
}

impl Rem for Expression {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Expression::Remainder(Box::new(self), Box::new(other))
    }
}

impl From<Integer> for Expression {
    fn from(integer: Integer) -> Self {
        Expression::Integer(integer)
    }
}

impl From<Rational> for Expression {
    fn from(rational: Rational) -> Self {
        Expression::Rational(rational, RationalRepresentation::Fraction)
    }
}

impl From<Complex> for Expression {
    fn from(complex: Complex) -> Self {
        Expression::Complex(complex, RationalRepresentation::Fraction)
    }
}

impl From<Vector> for Expression {
    fn from(vector: Vector) -> Self {
        Expression::Vector(vector)
    }
}

impl From<Matrix> for Expression {
    fn from(matrix: Matrix) -> Self {
        Expression::Matrix(matrix)
    }
}

impl From<bool> for Expression {
    fn from(boolean: bool) -> Self {
        Expression::Boolean(boolean)
    }
}

/// Returns an expression representing the variable with the given identifier.
pub fn var(identifier: impl Into<String>) -> Expression {
    Expression::Variable(identifier.into())
}

/// Returns an expression representing the evaluation of the given function at the given arguments.
pub fn fun(function: impl Into<Expression>, arguments: impl Into<Vec<Expression>>) -> Expression {
    Expression::Function(Box::new(function.into()), arguments.into())
}

/// Returns an expression representing the given integer.
pub fn int(integer: impl Into<Integer>) -> Expression {
    Expression::Integer(integer.into())
}

/// Returns an expression representing the rational number with
/// the given numerator and denominator, using fraction representation.
pub fn rat(numerator: impl Into<Integer>, denominator: impl Into<Integer>) -> Expression {
    Expression::Rational(
        Rational::new(numerator.into(), denominator.into()),
        RationalRepresentation::Fraction,
    )
}

/// Returns an expression representing the rational number with
/// the given numerator and denominator, using decimal representation,
/// falling back to fraction representation if the number cannot be
/// represented as a finite decimal.
pub fn ratd(numerator: impl Into<Integer>, denominator: impl Into<Integer>) -> Expression {
    Expression::Rational(
        Rational::new(numerator.into(), denominator.into()),
        RationalRepresentation::Decimal,
    )
}

/// Returns an expression representing the complex number with
/// real and imaginary parts being rational numbers described by
/// the given numerators and denominators, using fraction representation.
pub fn com(
    real_numerator: impl Into<Integer>,
    real_denominator: impl Into<Integer>,
    imaginary_numerator: impl Into<Integer>,
    imaginary_denominator: impl Into<Integer>,
) -> Expression {
    Expression::Complex(
        Complex::new(
            Rational::new(real_numerator.into(), real_denominator.into()),
            Rational::new(imaginary_numerator.into(), imaginary_denominator.into()),
        ),
        RationalRepresentation::Fraction,
    )
}

/// Returns an expression representing the complex number with
/// real and imaginary parts being rational numbers described by
/// the given numerators and denominators, using decimal representation,
/// falling back to fraction representation for parts that cannot be
/// represented as a finite decimal.
pub fn comd(
    real_numerator: impl Into<Integer>,
    real_denominator: impl Into<Integer>,
    imaginary_numerator: impl Into<Integer>,
    imaginary_denominator: impl Into<Integer>,
) -> Expression {
    Expression::Complex(
        Complex::new(
            Rational::new(real_numerator.into(), real_denominator.into()),
            Rational::new(imaginary_numerator.into(), imaginary_denominator.into()),
        ),
        RationalRepresentation::Decimal,
    )
}

/// Returns an expression representing the first expression raised to the power of the second.
pub fn pow(base: impl Into<Expression>, exponent: impl Into<Expression>) -> Expression {
    Expression::Power(Box::new(base.into()), Box::new(exponent.into()))
}

/// Returns an expression representing whether two expressions are equal.
pub fn eq(left: impl Into<Expression>, right: impl Into<Expression>) -> Expression {
    Expression::Equal(Box::new(left.into()), Box::new(right.into()))
}

/// Returns an expression representing whether two expressions are not equal.
pub fn ne(left: impl Into<Expression>, right: impl Into<Expression>) -> Expression {
    Expression::NotEqual(Box::new(left.into()), Box::new(right.into()))
}

/// Returns an expression representing whether the first expression is less than the second.
pub fn lt(left: impl Into<Expression>, right: impl Into<Expression>) -> Expression {
    Expression::LessThan(Box::new(left.into()), Box::new(right.into()))
}

/// Returns an expression representing whether the first expression is less than or equal to the second.
pub fn le(left: impl Into<Expression>, right: impl Into<Expression>) -> Expression {
    Expression::LessThanOrEqual(Box::new(left.into()), Box::new(right.into()))
}

/// Returns an expression representing whether the first expression is greater than the second.
pub fn gt(left: impl Into<Expression>, right: impl Into<Expression>) -> Expression {
    Expression::GreaterThan(Box::new(left.into()), Box::new(right.into()))
}

/// Returns an expression representing whether the first expression is greater than or equal to the second.
pub fn ge(left: impl Into<Expression>, right: impl Into<Expression>) -> Expression {
    Expression::GreaterThanOrEqual(Box::new(left.into()), Box::new(right.into()))
}

/// Returns an expression representing the logical conjunction (AND) of two expressions.
pub fn and(a: impl Into<Expression>, b: impl Into<Expression>) -> Expression {
    Expression::And(Box::new(a.into()), Box::new(b.into()))
}

/// Returns an expression representing the logical disjunction (OR) of two expressions.
pub fn or(a: impl Into<Expression>, b: impl Into<Expression>) -> Expression {
    Expression::Or(Box::new(a.into()), Box::new(b.into()))
}
