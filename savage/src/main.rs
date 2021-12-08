// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use std::collections::HashMap;

use rustyline::{error::ReadlineError, Editor};
use savage_core::expression::Expression;

fn main() {
    let mut editor = Editor::<()>::new();

    loop {
        match editor.readline("> ") {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                editor.add_history_entry(line);

                match line.parse::<Expression>() {
                    Ok(expression) => match expression.evaluate(HashMap::new()) {
                        Ok(result) => println!("{}", result),
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
}
