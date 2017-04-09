//! This module serves as the entry point into Xtensis's main binary.

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

extern crate clap;
extern crate xtensis;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_scope;

use slog_scope::logger;
use clap::{App, Arg, ArgMatches, SubCommand};
use xtensis::logging::init_logger;
use xtensis::utils::get_version;

fn retrieve_arguments() -> ArgMatches<'static> {
    App::new("xtensis-core")
        .version(get_version())
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("Extensible editing: screw limits.")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .required(false)
            .help("Set the level of logging verbosity"))
        .subcommand(SubCommand::with_name("spawn")
            .about("Spawn a new instance of xtensis-core.")
            .version(get_version())
            .author("Dom Rodriguez <shymega@shymega.org.uk>"))
        .get_matches()
}

fn main() {
    let cargs = retrieve_arguments();

    init_logger(cargs.clone());

    if cargs.subcommand_matches("spawn").is_some() {
        info!(logger(), "xtensis-core starting.. initialising ")
    }

    println!("Hello, XTENSIS!");
}
