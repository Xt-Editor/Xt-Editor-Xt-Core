//! This is the logging module for Xt.

// This file is part of Xt.

// This is the Xt text editor; it edits text.
// Copyright (C) 2016-2018  The Xt Developers

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

extern crate slog;
extern crate slog_term;

use std::sync::Mutex;

use slog::{Logger, Drain, Level};
use slog_term::{TermDecorator, FullFormat};

/// Logger initialistion function.
pub fn init_logger(module: & 'static str, lvl: Level) -> Logger {
    let decorator = TermDecorator::new()
        .build();

    let drain = FullFormat::new(decorator)
        .build()
        .fuse();

    let drain = slog::LevelFilter::new(drain, lvl)
        .fuse();

    let drain = Mutex::new(drain)
        .fuse();

    Logger::root(drain, o!("module" => module))
}
