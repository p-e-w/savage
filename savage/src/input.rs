// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021-2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use std::borrow::Cow;

use ansi_term::Style;
use lazy_static::lazy_static;
use regex::Regex;
use rustyline::{
    highlight::Highlighter,
    validate::{ValidationContext, ValidationResult, Validator},
    Result,
};
use rustyline_derive::{Completer, Helper, Hinter};
use savage_core::{expression::Expression, parse::ErrorReason};

enum TokenType {
    Literal,
    Variable,
    Operator,
    Bracket,
    Separator,
    Whitespace,
    Invalid,
}

fn tokenize(input: &str) -> Vec<(String, TokenType)> {
    use TokenType::*;

    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            &[
                r"(?P<literal>[0-9]+(?:\.[0-9]+)?|true|false)",
                r"(?P<variable>[a-zA-Z_][a-zA-Z0-9_]*)",
                r"(?P<operator>[+\-*/%^!=<>&|]+)",
                r"(?P<bracket>[()\[\]])",
                r"(?P<separator>,)",
                r"(?P<whitespace>\s+)",
            ]
            .join("|"),
        )
        .unwrap();
    }

    let mut tokens = Vec::new();

    let mut last_token_end = 0;

    for captures in REGEX.captures_iter(input) {
        let token_type = if captures.name("literal").is_some() {
            Literal
        } else if captures.name("variable").is_some() {
            Variable
        } else if captures.name("operator").is_some() {
            Operator
        } else if captures.name("bracket").is_some() {
            Bracket
        } else if captures.name("separator").is_some() {
            Separator
        } else if captures.name("whitespace").is_some() {
            Whitespace
        } else {
            unreachable!()
        };

        let token_range = captures.get(0).unwrap().range();

        if last_token_end < token_range.start {
            tokens.push((input[last_token_end..token_range.start].to_owned(), Invalid));
        }

        last_token_end = token_range.end;

        tokens.push((input[token_range].to_owned(), token_type));
    }

    if last_token_end < input.len() {
        tokens.push((input[last_token_end..].to_owned(), Invalid));
    }

    tokens
}

#[derive(Completer, Helper, Hinter)]
pub struct InputHelper {}

impl Highlighter for InputHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Cow::Owned(Style::new().bold().paint(prompt).to_string())
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        use ansi_term::Colour::*;
        use TokenType::*;

        let matching_pos = if pos < line.len() {
            let line_bytes = line.as_bytes();

            let mut matching_pos = None;

            'outer: for (open, close) in [(b'(', b')'), (b'[', b']')] {
                let (range, open, close): (Box<dyn Iterator<Item = usize>>, _, _) =
                    if line_bytes[pos] == open {
                        (Box::new(pos + 1..line_bytes.len()), open, close)
                    } else if line_bytes[pos] == close {
                        (Box::new((0..pos).rev()), close, open)
                    } else {
                        continue;
                    };

                let mut closes_needed = 1;

                for i in range {
                    if line_bytes[i] == open {
                        closes_needed += 1;
                    } else if line_bytes[i] == close {
                        closes_needed -= 1;

                        if closes_needed == 0 {
                            matching_pos = Some(i);
                            break 'outer;
                        }
                    }
                }
            }

            matching_pos
        } else {
            None
        };

        let mut highlighted_line = String::new();

        let mut token_pos = 0;

        for (token, token_type) in tokenize(line) {
            let mut style = match token_type {
                Literal => Cyan.into(),
                Variable => Green.into(),
                Operator => Purple.into(),
                Bracket => Style::new(),
                Separator => Style::new(),
                Whitespace => Style::new(),
                Invalid => Red.into(),
            };

            if Some(token_pos) == matching_pos {
                style = style.bold();
            }

            token_pos += token.len();

            // https://github.com/rust-lang/rust-clippy/issues/9317
            #[allow(clippy::unnecessary_to_owned)]
            highlighted_line.push_str(&style.paint(token).to_string());
        }

        Cow::Owned(highlighted_line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        true
    }
}

impl Validator for InputHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        // This implementation distinguishes only between "incomplete"
        // (unexpected end of input) and "valid" (everything else),
        // because actual input validation is performed by the REPL
        // as part of the regular input processing step.
        let input = ctx.input();

        if input.trim().is_empty() || input.ends_with('\n') {
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
