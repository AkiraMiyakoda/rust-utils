// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{borrow::Cow, io::Write};

use chrono::Local;
use env_logger::{
    Builder, DEFAULT_FILTER_ENV,
    fmt::style::{AnsiColor, Effects, Style},
};
use log::Level;

pub use log;

const DEAFULT_LEVEL: &str = "error";

pub fn init() {
    let level: Cow<_> =
        std::env::var(DEFAULT_FILTER_ENV).map_or(DEAFULT_LEVEL.into(), |v| v.into());

    Builder::new()
        .format(|buf, record| {
            let styles = {
                let (color, effect) = match record.level() {
                    Level::Error => (AnsiColor::Red, Effects::BOLD),
                    Level::Warn => (AnsiColor::Yellow, Effects::BOLD),
                    Level::Info => (AnsiColor::Green, Effects::new()),
                    Level::Debug => (AnsiColor::Blue, Effects::new()),
                    Level::Trace => (AnsiColor::White, Effects::new()),
                };

                (
                    Style::new().fg_color(Some(color.into())).effects(effect),
                    Style::new().effects(effect),
                )
            };

            writeln!(
                buf,
                "{timestamp} [{st0}{level:<5}{st0:#}] {st1}{args}{st1:#}",
                timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f %Z"),
                st0 = styles.0,
                st1 = styles.1,
                level = record.level(),
                args = record.args(),
            )
        })
        .parse_filters(level.as_ref())
        .init();
}
