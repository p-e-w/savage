// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use num::{Signed, Zero};

/// Arbitrary-precision integer.
pub type Integer = num::bigint::BigInt;

/// Arbitrary-precision rational number.
pub type Rational = num::rational::Ratio<Integer>;

/// Arbitrary-precision complex number (i.e. real and imaginary parts are arbitrary-precision rational numbers).
pub type Complex = num::complex::Complex<Rational>;

/// Column vector with expressions as components.
pub type Vector = nalgebra::DVector<Expression>;

/// Column-major matrix with expressions as components.
pub type Matrix = nalgebra::DMatrix<Expression>;

/// Preferred representation when printing a rational number.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RationalRepresentation {
    /// Fraction (numerator/denominator).
    Fraction,
    /// Decimal, falling back to fraction representation
    /// if the number cannot be represented as a finite decimal.
    Decimal,
}

/// Symbolic expression.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expression {
    /// Integer.
    Integer(Integer),
    /// Rational number with preferred representation.
    Rational(Rational, RationalRepresentation),
    /// Complex number with preferred representation for real and imaginary parts.
    Complex(Complex, RationalRepresentation),
    /// Column vector.
    Vector(Vector),
    /// Column-major matrix.
    Matrix(Matrix),
    /// Sum of two expressions.
    Sum(Box<Expression>, Box<Expression>),
    /// Difference of two expressions.
    Difference(Box<Expression>, Box<Expression>),
    /// Product of two expressions.
    Product(Box<Expression>, Box<Expression>),
    /// Quotient of two expressions.
    Quotient(Box<Expression>, Box<Expression>),
    /// Remainder of the Euclidean division of the first expression by the second.
    Remainder(Box<Expression>, Box<Expression>),
    /// The first expression raised to the power of the second.
    Power(Box<Expression>, Box<Expression>),
    /// Whether two expressions are equal.
    Equal(Box<Expression>, Box<Expression>),
    /// Whether two expressions are not equal.
    NotEqual(Box<Expression>, Box<Expression>),
    /// Whether the first expression is less than the second.
    LessThan(Box<Expression>, Box<Expression>),
    /// Whether the first expression is less than or equal to the second.
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    /// Whether the first expression is greater than the second.
    GreaterThan(Box<Expression>, Box<Expression>),
    /// Whether the first expression is greater than or equal to the second.
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),
    /// Absolute value of an expression.
    AbsoluteValue(Box<Expression>),
}

/// Associativity of an operator expression.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum Associativity {
    /// `a OP b OP c == (a OP b) OP c`.
    LeftAssociative,
    /// `a OP b OP c == a OP (b OP c)`.
    RightAssociative,
    /// `a OP b OP c == (a OP b) OP c == a OP (b OP c)`.
    Associative,
}

impl Expression {
    /// Returns the precedence (as an integer intended for comparison)
    /// and associativity of the expression. For non-operator expressions,
    /// to which the concept of associativity doesn't apply, `Associative`
    /// is returned.
    pub(crate) fn precedence_and_associativity(&self) -> (isize, Associativity) {
        use Associativity::*;
        use Expression::*;

        match self {
            Integer(n) => {
                if n.is_negative() {
                    (2, Associative)
                } else {
                    (isize::MAX, Associative)
                }
            }
            Rational(x, _) => {
                if self.to_string().contains("/") {
                    (2, LeftAssociative)
                } else if x.is_negative() {
                    (2, Associative)
                } else {
                    (isize::MAX, Associative)
                }
            }
            Complex(z, _) => {
                if !z.re.is_zero() && !z.im.is_zero() {
                    if self.to_string().contains("+") {
                        (1, Associative)
                    } else {
                        (1, LeftAssociative)
                    }
                } else if self.to_string().contains("/") {
                    (2, LeftAssociative)
                } else if z.re.is_negative() || !z.im.is_zero() {
                    (2, Associative)
                } else {
                    (isize::MAX, Associative)
                }
            }
            Vector(_) => (isize::MAX, Associative),
            Matrix(_) => (isize::MAX, Associative),
            Sum(_, _) => (1, Associative),
            Difference(_, _) => (1, LeftAssociative),
            Product(_, _) => (2, Associative),
            Quotient(_, _) => (2, LeftAssociative),
            Remainder(_, _) => (2, LeftAssociative),
            Power(_, _) => (3, RightAssociative),
            Equal(_, _) => (0, Associative),
            NotEqual(_, _) => (0, Associative),
            LessThan(_, _) => (0, Associative),
            LessThanOrEqual(_, _) => (0, Associative),
            GreaterThan(_, _) => (0, Associative),
            GreaterThanOrEqual(_, _) => (0, Associative),
            AbsoluteValue(_) => (isize::MAX, Associative),
        }
    }
}
