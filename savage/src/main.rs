// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021-2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

mod command;
mod help;
mod input;

use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::FromIterator,
    rc::Rc,
};

use ansi_term::Style;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use directories::ProjectDirs;
use lazy_static::lazy_static;
use rustyline::{error::ReadlineError, highlight::Highlighter, Editor};
use savage_core::{
    evaluate::{default_context, Error as EvaluateError},
    expression::{Expression, Vector},
    parse::{Error as ParseError, ErrorReason},
};

use crate::{
    command::Command,
    help::{show_help, FUNCTION_HELP_TEXTS, HELP_TEXT},
    input::InputHelper,
};

lazy_static! {
    static ref RESERVED_IDENTIFIERS: HashSet<String> =
        HashSet::from(["true", "false", "out"].map(str::to_owned));
}

fn format_parse_error(error: ParseError) -> Report {
    // Heavily based on https://github.com/zesterer/chumsky/blob/463226372cf293d45bd5df52bf25d5028243066e/examples/json.rs#L114-L173
    let message = if let ErrorReason::Custom(message) = error.reason() {
        message.clone()
    } else {
        format!(
            "{}, expected {}",
            if error.found().is_some() {
                "Unexpected token"
            } else {
                "Unexpected end of input"
            },
            if error.expected().len() == 0 {
                "something else".to_string()
            } else {
                error
                    .expected()
                    .map(|expected| match expected {
                        Some(expected) => expected.to_string(),
                        None => "end of input".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        )
    };

    let report = Report::build(ReportKind::Error, (), error.span().start)
        .with_message(message)
        .with_label(
            Label::new(error.span())
                .with_message(match error.reason() {
                    ErrorReason::Custom(message) => message.clone(),
                    _ => format!(
                        "Unexpected {}",
                        error
                            .found()
                            .map(|c| format!("token {}", c.fg(Color::Red)))
                            .unwrap_or_else(|| "end of input".to_string()),
                    ),
                })
                .with_color(Color::Red),
        );

    let report = match error.reason() {
        ErrorReason::Unclosed { span, delimiter } => report.with_label(
            Label::new(span.clone())
                .with_message(format!(
                    "Unclosed delimiter {}",
                    delimiter.fg(Color::Yellow),
                ))
                .with_color(Color::Yellow),
        ),
        ErrorReason::Unexpected => report,
        ErrorReason::Custom(_) => report,
    };

    report.finish()
}

fn main() {
    use crate::command::Command::*;

    let history_path = ProjectDirs::from("com.worldwidemann", "", "Savage")
        .expect("unable to locate data directory")
        .data_dir()
        .join("history");

    let mut editor = Editor::new();

    editor.set_helper(Some(InputHelper {}));

    editor.load_history(&history_path).ok();

    println!(
        "Savage Computer Algebra System {}",
        env!("CARGO_PKG_VERSION"),
    );

    println!(
        "Enter {} for help, press {} to quit, {} to cancel evaluation",
        Style::new().bold().paint("?"),
        Style::new().bold().paint("Ctrl+D"),
        Style::new().bold().paint("Ctrl+C"),
    );

    let mut outputs = Vec::new();

    let mut context = default_context();

    context.insert(
        "out".to_owned(),
        Expression::Vector(Vector::from_vec(outputs.clone())),
    );

    'outer: loop {
        println!();

        match editor.readline("in: ") {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                editor.add_history_entry(line);

                match line.parse::<Command>() {
                    Ok(EvaluateExpression(expression)) => match expression.evaluate(&context) {
                        Ok(output) => {
                            println!(
                                "{}{}",
                                Style::new()
                                    .bold()
                                    .paint(format!("out[{}]: ", outputs.len())),
                                editor
                                    .helper()
                                    .unwrap()
                                    .highlight(&output.to_string(), usize::MAX),
                            );

                            outputs.push(output);

                            context.insert(
                                "out".to_owned(),
                                Expression::Vector(Vector::from_vec(outputs.clone())),
                            );
                        }
                        Err(error) => println!("Error: {:#?}", error),
                    },
                    Ok(DefineVariable(identifier, expression)) => {
                        if RESERVED_IDENTIFIERS.contains(&identifier) {
                            println!("Error: \"{}\" is a reserved identifier and cannot be used as a variable name.", identifier);
                            continue;
                        }

                        match expression.evaluate(&context) {
                            Ok(expression) => {
                                let variables = expression.variables();

                                if !variables.is_empty() {
                                    println!("Error: The assigned expression contains the undefined variable(s) {}.", Vec::from_iter(variables).join(", "));
                                    continue;
                                }

                                context.insert(identifier, expression);
                            }
                            Err(error) => println!("Error: {:#?}", error),
                        }
                    }
                    Ok(DefineFunction(identifier, argument_identifiers, expression)) => {
                        if RESERVED_IDENTIFIERS.contains(&identifier) {
                            println!("Error: \"{}\" is a reserved identifier and cannot be used as a function name.", identifier);
                            continue;
                        }

                        let mut inner_context = context.clone();

                        for argument_identifier in &argument_identifiers {
                            if RESERVED_IDENTIFIERS.contains(argument_identifier) {
                                println!("Error: \"{}\" is a reserved identifier and cannot be used as an argument name.", argument_identifier);
                                continue 'outer;
                            }

                            if argument_identifiers
                                .iter()
                                .filter(|&id| id == argument_identifier)
                                .count()
                                > 1
                            {
                                println!(
                                    "Error: The name \"{}\" is used for more than one argument.",
                                    argument_identifier,
                                );
                                continue 'outer;
                            }

                            inner_context.remove(argument_identifier);
                        }

                        match expression.evaluate(&inner_context) {
                            Ok(expression) => {
                                let mut variables = expression.variables();

                                for argument_identifier in &argument_identifiers {
                                    variables.remove(argument_identifier);
                                }

                                if !variables.is_empty() {
                                    println!("Error: The assigned expression contains the undefined variable(s) {}.", Vec::from_iter(variables).join(", "));
                                    continue;
                                }

                                context.insert(
                                    identifier.clone(),
                                    Expression::Function(
                                        identifier,
                                        Rc::new(move |self_expression, arguments, _| {
                                            if arguments.len() != argument_identifiers.len() {
                                                return Err(
                                                    EvaluateError::InvalidNumberOfArguments {
                                                        expression: self_expression.clone(),
                                                        min_number: argument_identifiers.len(),
                                                        max_number: argument_identifiers.len(),
                                                        given_number: arguments.len(),
                                                    },
                                                );
                                            }

                                            // Both the default context and the outer context the function is being
                                            // evaluated in can be ignored, since it was already checked that the
                                            // expression contains no variables other than the argument identifiers.
                                            let mut context = HashMap::new();

                                            for (identifier, argument) in
                                                argument_identifiers.iter().zip(arguments)
                                            {
                                                context
                                                    .insert(identifier.clone(), argument.clone());
                                            }

                                            expression.evaluate(&context)
                                        }),
                                    ),
                                );
                            }
                            Err(error) => println!("Error: {:#?}", error),
                        }
                    }
                    Ok(ShowHelp(function_name)) => {
                        if let Some(function_name) = function_name {
                            if let Some(function_help_text) =
                                FUNCTION_HELP_TEXTS.get(&function_name)
                            {
                                show_help(function_help_text.clone()).expect("unable to show help");
                            } else {
                                println!(
                                    "Error: No help text available for the function {}.",
                                    function_name,
                                );
                            }
                        } else {
                            show_help(HELP_TEXT.clone()).expect("unable to show help");
                        }
                    }
                    Err(errors) => {
                        for error in errors {
                            format_parse_error(error)
                                .print(Source::from(line))
                                .expect("unable to print parse error");
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(error) => {
                println!("Error: {:#?}", error);
                break;
            }
        }
    }

    fs::create_dir_all(
        history_path
            .parent()
            .expect("unable to determine parent directory of history file"),
    )
    .expect("unable to create data directory");

    editor
        .save_history(&history_path)
        .expect("unable to save input history");
}
