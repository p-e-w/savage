// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

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
#[derive(Clone, Debug)]
pub enum RationalRepresentation {
    /// Standard fraction.
    Fraction,
    /// Continued fraction.
    ContinuedFraction,
    /// Decimal, falling back to standard fraction representation
    /// if the number cannot be exactly represented as a finite decimal.
    Decimal,
}

/// Symbolic expression.
#[derive(Clone, Debug)]
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
