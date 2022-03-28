//! This is the logging module for Xt.

use slog::{o, Drain, Logger};
use slog_term::{FullFormat, TermDecorator};
use std::sync::Mutex;

/// Logger initialistion function.
pub fn init_logger() -> Option<Logger> {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Mutex::from(drain).fuse();

    Some(Logger::root(drain, o!()))
}
