// Copyright © 2023 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{borrow::Cow, io::Write};

use chrono::Local;
use env_logger::{
    fmt::style::{AnsiColor, Effects, Style},
    Builder, DEFAULT_FILTER_ENV,
};
use log::Level;

const DEAFULT_LEVEL: &str = "error";

pub fn init_logger() {
    let level: Cow<str> = std::env::var(DEFAULT_FILTER_ENV)
        .map(|v| v.into())
        .unwrap_or(DEAFULT_LEVEL.into());

    Builder::new()
        .format(|buf, record| {
            let (color, effect) = match record.level() {
                Level::Error => (AnsiColor::Red, Effects::BOLD),
                Level::Warn => (AnsiColor::Yellow, Effects::BOLD),
                Level::Info => (AnsiColor::Green, Effects::new()),
                Level::Debug => (AnsiColor::Blue, Effects::new()),
                Level::Trace => (AnsiColor::White, Effects::new()),
            };
            let level_style = Style::new().fg_color(Some(color.into())).effects(effect);
            let args_style = Style::new().effects(effect);

            writeln!(
                buf,
                "{timestamp} [{level_style}{level:<5}{level_style:#}] {args_style}{args}{args_style:#}",
                timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.6f %Z"),
                level = record.level(),
                args = record.args(),
            )
        })
        .parse_filters(level.as_ref())
        .init();
}
