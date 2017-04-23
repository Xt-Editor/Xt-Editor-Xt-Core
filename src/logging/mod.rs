//! this is the logging module for xtensis.

// this file is part of xtensis.

// this is the xtensis text editor; it edits text.
// copyright (c) 2016-2017  the xtensis developers

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

extern crate time;
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use self::slog_term::{TermDecorator, FullFormat};
use slog::{Drain, Logger};
use slog_async::Async;

use utils::{get_pkg_name, get_version};

/// Initialise the logger.
pub fn init_logger() -> Logger {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();


    let root_logger = Logger::root(drain,
                                   o!("version" => get_version(),
                                      "app" => get_pkg_name()));

    debug!(root_logger, "Logger initialised.");

    return root_logger;
}
