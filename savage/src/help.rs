// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021-2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use std::{
    collections::HashMap,
    io::{stdout, Write},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyEvent},
    queue,
    style::Stylize,
    terminal::{
        self, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use lazy_static::lazy_static;
use savage_core::functions::functions;
use termimad::{Area, Error, MadSkin, MadView};

const HELP_HEADER: &str = include_str!("../help/header.md");
const HELP_FOOTER: &str = include_str!("../help/footer.md");

lazy_static! {
    pub static ref FUNCTION_HELP_TEXTS: HashMap<String, String> = {
        let mut texts = HashMap::new();

        for function in functions() {
            let metadata = function.metadata;

            let text = format!(
                "**{}** - {}\n\n*Syntax:*\n```\n{}({})\n```\n\n*Examples:*\n```\n{}\n```\n\n*Categories:*\n{}\n",
                metadata.name,
                metadata.description,
                metadata.name,
                metadata
                    .parameters
                    .iter()
                    .map(|p| format!("{:?}", p))
                    .collect::<Vec<_>>()
                    .join(", "),
                metadata
                    .examples
                    .iter()
                    .map(|(i, o)| format!("in: {}\nout: {}", i, o))
                    .collect::<Vec<_>>()
                    .join("\n\n"),
                metadata.categories.join(", "),
            );

            texts.insert(metadata.name.to_owned(), text);
        }

        texts
    };
    pub static ref HELP_TEXT: String = {
        let mut text = String::from(HELP_HEADER);

        text.push_str("\n---\n\n");

        for function in functions() {
            text.push_str(&FUNCTION_HELP_TEXTS[function.metadata.name]);
            text.push_str("\n---\n\n");
        }

        text.push('\n');
        text.push_str(HELP_FOOTER);
        text.push('\n');

        text
    };
}

fn view_area() -> Area {
    let mut area = Area::full_screen();

    area.pad_for_max_width(120);

    // Make space for bottom bar.
    area.height -= 1;

    area
}

pub fn show_help(text: String) -> Result<(), Error> {
    // Based on https://github.com/Canop/termimad/blob/5ab13e600f05c0217e270181dd5d9288210f893f/examples/scrollable/main.rs
    use crossterm::event::KeyCode::*;

    let mut stdout = stdout();

    queue!(stdout, EnterAlternateScreen, DisableLineWrap, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    let mut view = MadView::from(text, view_area(), MadSkin::default());

    loop {
        view.write_on(&mut stdout)?;

        print!(
            "\n\r{}{}{}{}{}{}{}{}",
            "Press ".reverse(),
            "\u{2191}".bold().reverse(),
            " and ".reverse(),
            "\u{2193}".bold().reverse(),
            " to scroll, ".reverse(),
            "Q".bold().reverse(),
            " to quit".reverse(),
            " ".repeat(view_area().width as usize).reverse(),
        );

        stdout.flush()?;

        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up | Char('k') => view.try_scroll_lines(-1),
                Down | Char('j') | Enter => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown | Char(' ') => view.try_scroll_pages(1),
                Char('q') | Esc => break,
                _ => {}
            },
            Ok(Event::Resize(..)) => {
                queue!(stdout, terminal::Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    queue!(stdout, cursor::Show, EnableLineWrap, LeaveAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}
