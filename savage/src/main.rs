// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

mod input;

use std::{collections::HashMap, fs};

use ansi_term::Style;
use directories::ProjectDirs;
use rustyline::{error::ReadlineError, highlight::Highlighter, Editor};
use savage_core::expression::{Expression, Vector};

use crate::input::InputHelper;

fn main() {
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

                match line.parse::<Expression>() {
                    Ok(expression) => match expression.evaluate(context.clone()) {
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
                    Err(error) => println!("Error: {:#?}", error),
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
