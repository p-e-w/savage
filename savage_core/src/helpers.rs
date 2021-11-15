// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

//! Operators, conversions, and helper functions to make working with expressions easier.

use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::expression::{
    Complex, Expression, Integer, Matrix, Rational, RationalRepresentation, Vector,
};

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
