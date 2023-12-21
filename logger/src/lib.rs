// Copyright © 2023 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use env_logger::{fmt::Color, Builder};
use log::Level;
use std::io::Write;

const DEAFULT_LEVEL: &str = "error";

pub fn init_logger() {
    let level = match std::env::var("RUST_LOG") {
        Ok(level) => level,
        Err(_) => String::from(DEAFULT_LEVEL),
    };

    Builder::new()
        .format(|buf, record| {
            let (color, bold) = match record.level() {
                Level::Error => (Color::Red, true),
                Level::Warn => (Color::Yellow, true),
                Level::Info => (Color::Green, false),
                Level::Debug => (Color::Blue, false),
                Level::Trace => (Color::White, false),
            };

            writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f %Z"),
                buf.style()
                    .set_color(color)
                    .set_bold(bold)
                    .value(record.level()),
                buf.style().set_bold(bold).value(record.args()),
            )
        })
        .parse_filters(&level)
        .init();
}
