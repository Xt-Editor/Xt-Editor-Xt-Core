//! This is the logging module for Xt.

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::{Drain, Level, LevelFilter, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

/// Logger initialistion function.
pub fn init_logger(module: &'static str, lvl: Level) -> Logger {
    let decorator = TermDecorator::new()
        .build();

    let drain = FullFormat::new(decorator)
        .build()
        .fuse();

    let drain = LevelFilter::new(drain, lvl)
        .fuse();

    let drain = Async::new(drain)
        .build()
        .fuse();

    Logger::root(drain, o!("module" => module))
}
