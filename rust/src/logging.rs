//! this is the logging module for Xt.

// this file is part of Xt.

// this is the Xt text editor; it edits text.
// copyright (c) 2016-2017  the Xt developers

// this program is free software: you can redistribute it and/or
// modify it under the terms of the gnu general public license as
// published by the free software foundation, either version 3 of the
// license, or (at your option) any later version.

// this program is distributed in the hope that it will be useful, but
// without any warranty; without even the implied warranty of
// merchantability or fitness for a particular purpose.  see the gnu
// general public license for more details.

// you should have received a copy of the gnu general public license
// along with this program.  if not, see
// <http://www.gnu.org/licenses/>.

extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

/// Initialise the logger.
pub fn init_logger() -> Logger {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}
