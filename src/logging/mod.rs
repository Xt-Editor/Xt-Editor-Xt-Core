//! This is the logging module for Xtensis.

// This file is part of Xtensis.

// This is the Xtensis text editor; it edits text.
// Copyright (C) 2016-2017  The Xtensis Developers

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

extern crate time;
extern crate slog;
extern crate slog_term;
extern crate slog_scope;
extern crate clap;

use slog::DrainExt;
use utils::get_version;
use self::clap::ArgMatches;
use slog::Level;

/// Initialise the logger.

pub fn init_logger(cargs: ArgMatches) -> slog::Logger {
    let log_level = match cargs.occurrences_of("verbose") {
        0 => Level::Warning,
        1 => Level::Info,
        2 => Level::Debug,
        3 | _ => Level::Trace,
    };

    let streamer = slog_term::streamer().build().fuse();
    let drain = slog::level_filter(log_level, streamer);
    let root_log = slog::Logger::root(drain, o!("version" => get_version()));

    info!(root_log, "Logging initialised");

    return root_log;
}
