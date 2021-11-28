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

impl RationalRepresentation {
    /// Returns the preferred representation for the result of an operation
    /// on two numbers with representations `self` and `other`.
    pub(crate) fn merge(self, other: Self) -> Self {
        use RationalRepresentation::*;

        if self == Decimal || other == Decimal {
            Decimal
        } else {
            Fraction
        }
    }
}

/// Symbolic expression.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expression {
    /// Variable with identifier.
    Variable(String),
    /// Function evaluation with arguments.
    Function(Box<Expression>, Vec<Expression>),
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
    /// Boolean value.
    Boolean(bool),
    /// Arithmetic negation of an expression.
    Negation(Box<Expression>),
    /// Logical negation (NOT) of an expression.
    Not(Box<Expression>),
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
    /// Logical conjunction (AND) of two expressions.
    And(Box<Expression>, Box<Expression>),
    /// Logical disjunction (OR) of two expressions.
    Or(Box<Expression>, Box<Expression>),
}

/// Basic expression type designed to make evaluating expressions easier.
#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum Type {
    /// Number with preferred representation for rational parts.
    Number(Complex, RationalRepresentation),
    /// Column-major matrix.
    Matrix(Matrix),
    /// Boolean expression with value (if available).
    Boolean(Option<bool>),
    /// Arithmetic expression (in particular, this expression does *not* have a boolean value).
    Arithmetic,
    /// Expression that cannot be assigned to any of the above types with certainty.
    Unknown,
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
    /// Returns the basic type of the expression.
    pub(crate) fn typ(&self) -> Type {
        use Expression::*;
        use RationalRepresentation::*;
        use Type::{Arithmetic, Boolean as Bool, Matrix as Mat, Number as Num, Unknown};

        match self {
            Variable(_) => Unknown,
            Function(_, _) => Unknown,
            Integer(n) => Num(self::Rational::from_integer(n.clone()).into(), Fraction),
            Rational(x, representation) => Num(x.into(), *representation),
            Complex(z, representation) => Num(z.clone(), *representation),
            Vector(v) => Mat(self::Matrix::from_columns(&[v.clone()])),
            Matrix(m) => Mat(m.clone()),
            Boolean(boolean) => Bool(Some(*boolean)),
            Negation(_) => Arithmetic,
            Not(_) => Bool(None),
            Sum(_, _) => Arithmetic,
            Difference(_, _) => Arithmetic,
            Product(_, _) => Arithmetic,
            Quotient(_, _) => Arithmetic,
            Remainder(_, _) => Arithmetic,
            Power(_, _) => Arithmetic,
            Equal(_, _) => Bool(None),
            NotEqual(_, _) => Bool(None),
            LessThan(_, _) => Bool(None),
            LessThanOrEqual(_, _) => Bool(None),
            GreaterThan(_, _) => Bool(None),
            GreaterThanOrEqual(_, _) => Bool(None),
            And(_, _) => Bool(None),
            Or(_, _) => Bool(None),
        }
    }

    /// Returns the precedence (as an integer intended for comparison)
    /// and associativity of the expression. For unary or non-operator
    /// expressions, to which the concept of associativity doesn't apply,
    /// `Associative` is returned.
    pub(crate) fn precedence_and_associativity(&self) -> (isize, Associativity) {
        use Associativity::*;
        use Expression::*;

        match self {
            Variable(_) => (isize::MAX, Associative),
            Function(_, _) => (5, Associative),
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
            Boolean(_) => (isize::MAX, Associative),
            Negation(_) => (3, Associative),
            Not(_) => (3, Associative),
            Sum(_, _) => (1, Associative),
            Difference(_, _) => (1, LeftAssociative),
            Product(_, _) => (2, Associative),
            Quotient(_, _) => (2, LeftAssociative),
            Remainder(_, _) => (2, LeftAssociative),
            Power(_, _) => (4, RightAssociative),
            Equal(_, _) => (0, Associative),
            NotEqual(_, _) => (0, Associative),
            LessThan(_, _) => (0, Associative),
            LessThanOrEqual(_, _) => (0, Associative),
            GreaterThan(_, _) => (0, Associative),
            GreaterThanOrEqual(_, _) => (0, Associative),
            And(_, _) => (-1, Associative),
            Or(_, _) => (-2, Associative),
        }
    }

    /// Returns the precedence (as an integer intended for comparison) of the expression.
    pub(crate) fn precedence(&self) -> isize {
        self.precedence_and_associativity().0
    }

    /// Returns the associativity of the expression.
    /// For unary or non-operator expressions, to which the concept
    /// of associativity doesn't apply, `Associative` is returned.
    pub(crate) fn associativity(&self) -> Associativity {
        self.precedence_and_associativity().1
    }
}
