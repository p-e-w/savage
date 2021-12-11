// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use rustyline::{
    validate::{ValidationContext, ValidationResult, Validator},
    Result,
};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};
use savage_core::{expression::Expression, parse::ErrorReason};

#[derive(Completer, Helper, Highlighter, Hinter)]
pub struct InputHelper {}

impl Validator for InputHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        // This implementation distinguishes only between "incomplete"
        // (unexpected end of input) and "valid" (everything else),
        // because actual input validation is performed by the REPL
        // as part of the regular input processing step.
        let input = ctx.input();

        if input.trim().is_empty() || input.ends_with("\n") {
            return Ok(ValidationResult::Valid(None));
        }

        if let Err(errors) = input.parse::<Expression>() {
            for error in errors {
                if error.reason() == &ErrorReason::Unexpected && error.found() == None {
                    return Ok(ValidationResult::Incomplete);
                }
            }
        }

        Ok(ValidationResult::Valid(None))
    }
}
