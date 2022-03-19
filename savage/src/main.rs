// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021-2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

mod command;
mod input;

use std::{collections::HashMap, fs};

use ansi_term::Style;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use directories::ProjectDirs;
use rustyline::{error::ReadlineError, highlight::Highlighter, Editor};
use savage_core::{
    expression::{Expression, Vector},
    parse::{Error, ErrorReason},
};

use crate::{command::Command, input::InputHelper};

fn format_parse_error(error: Error) -> Report {
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

    let mut context = HashMap::new();

    context.insert(
        "out".to_owned(),
        Expression::Vector(Vector::from_vec(outputs.clone())),
    );

    loop {
        println!();

        match editor.readline("in: ") {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                editor.add_history_entry(line);

                match line.parse::<Command>() {
                    Ok(EvaluateExpression(expression)) => {
                        match expression.evaluate(context.clone()) {
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
                        }
                    }
                    Ok(DefineVariable(identifier, expression)) => {
                        println!(
                            "Define variable {} as {}: Not implemented yet.",
                            identifier, expression,
                        );
                    }
                    Ok(DefineFunction(identifier, argument_identifiers, expression)) => {
                        println!(
                            "Define function {} with arguments [{}] as {}: Not implemented yet.",
                            identifier,
                            argument_identifiers.join(", "),
                            expression,
                        );
                    }
                    Ok(ShowHelp(function_name)) => {
                        println!(
                            "Show help for {}: Not implemented yet.",
                            function_name.unwrap_or("all functions".to_owned()),
                        );
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
