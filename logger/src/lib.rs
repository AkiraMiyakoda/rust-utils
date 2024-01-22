// Copyright © 2023 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use env_logger::{
    fmt::style::{AnsiColor, Effects, Style},
    Builder, DEFAULT_FILTER_ENV,
};
use log::Level;
use std::{borrow::Cow, io::Write};

const DEAFULT_LEVEL: &str = "error";

pub fn init_logger() {
    let level = std::env::var(DEFAULT_FILTER_ENV)
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed(DEAFULT_LEVEL));

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
                "{} [{}{:<5}{}] {}{}{}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f %Z"),
                level_style.render(),
                record.level(),
                level_style.render_reset(),
                args_style.render(),
                record.args(),
                args_style.render_reset(),
            )
        })
        .parse_filters(level.as_ref())
        .init();
}
