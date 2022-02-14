// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021-2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

mod logic;

use std::rc::Rc;

use savage_macros::functions;

use crate::expression::{Expression, Function as FunctionImplementation};

/// Function parameter.
pub enum Parameter {
    /// Any symbolic expression.
    Expression,
    /// Integer expression, or an expression that can be interpreted as an integer.
    Integer,
    /// Rational number expression, or an expression that can be interpreted as a rational number.
    Rational,
    /// Complex number expression, or an expression that can be interpreted as a complex number.
    Complex,
    /// Vector expression, or an expression that can be interpreted as a vector.
    Vector,
    /// Matrix expression, or an expression that can be interpreted as a matrix.
    Matrix,
    /// Boolean expression, or an expression that can be interpreted as a boolean value.
    Boolean,
}

/// Metadata associated with a function.
pub struct Metadata {
    /// Name used to represent the function (also, default identifier for invoking the function).
    pub name: &'static str,
    /// Human-readable description of the function.
    pub description: &'static str,
    /// Parameters expected by the function, in the expected order.
    pub parameters: &'static [Parameter],
    /// Usage examples for the function, as pairs of REPL input and output.
    pub examples: &'static [(&'static str, &'static str)],
    /// Categories associated with the function.
    pub categories: &'static [&'static str],
}

/// Function definition.
pub struct Function {
    /// Metadata associated with the function.
    pub metadata: Metadata,
    /// Implementation of the function.
    pub implementation: Rc<FunctionImplementation>,
}

/// Returns a regular function implementation that type-checks its arguments
/// based on the given `parameters` and then invokes the given function `proxy`.
fn wrap_proxy(
    parameters: &'static [Parameter],
    proxy: impl Fn(&[Expression]) -> Result<Expression, Expression> + 'static,
) -> Rc<FunctionImplementation> {
    use crate::evaluate::Error::*;
    use crate::expression::Type::{Arithmetic, Boolean as Bool, Unknown};
    use Parameter::*;

    Rc::new(move |expression, arguments, _| {
        if arguments.len() != parameters.len() {
            return Err(InvalidNumberOfArguments {
                expression: expression.clone(),
                min_number: parameters.len(),
                max_number: parameters.len(),
                given_number: arguments.len(),
            });
        }

        for (argument, parameter) in arguments.iter().zip(parameters) {
            if let (
                Bool(None) | Arithmetic | Unknown,
                Integer | Rational | Complex | Vector | Matrix | Boolean,
            ) = (argument.typ(), parameter)
            {
                return Ok(expression.clone());
            }
        }

        proxy(arguments).map_err(|argument| InvalidArgument {
            expression: expression.clone(),
            argument,
        })
    })
}

/// Returns all available functions.
pub fn functions() -> Vec<Function> {
    functions!(logic::and)
}
