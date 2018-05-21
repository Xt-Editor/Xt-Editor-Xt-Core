//! This module serves as the entry point into Xt's main binary.

// This file is part of Xt.

// This is the Xt text editor; it edits text.
// Copyright (C) 2016-2017  The Xt Developers

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
extern crate xt_core as xt;

#[macro_use]
extern crate slog;
extern crate slog_term;

use clap::{App, Arg, ArgMatches, SubCommand};
use xt::logging::init_logger;

use xt::server::buffer::Buffer;
use xt::server::modes::major_mode::MajorMode;
use xt::server::modes::minor_mode::MinorMode;
use xt::server::workspace::Workspace;

#[cfg(feature = "logo")]
use std::thread::sleep;

#[cfg(feature = "logo")]
use std::time::Duration;

fn retrieve_arguments() -> ArgMatches<'static> {
    App::new("xt-core")
        .version("0.1.0")
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("Core backend for Xt.")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .required(false)
                .help("Set the level of logging verbosity"),
        )
        .subcommand(
            SubCommand::with_name("spawn").help("Spawn a instance of Xt"),
        )
        .get_matches()
}

#[cfg(feature = "logo")]
fn print_logo() {
    let logo = include_str!("../../data/logo.txt");

    for line in logo.lines() {
        /* print line */
        println!("{}", line);

        /* sleep for specified time */
        sleep(Duration::from_millis(115));
    }
}

fn main() {
    let _args = retrieve_arguments();

    let log = init_logger();

    info!(log, "Xt (core) loading..");

    /* print logo? */
    #[cfg(feature = "logo")]
    print_logo();

    warn!(
        log,
        "Xt (core) has no configuration file. Reverting to defaults."
    );
    error!(log, "Xt (core) is not ready for deployment. Halt.");

    info!(log, "Initialise buffer creation.");

    let mut ws = Workspace::new("Default".to_string());
    let mam = MajorMode::new("fundamental-mode".to_string());
    let mut mim: Vec<MinorMode> = Vec::new();

    let mim1 = MinorMode::new("auto-wrap-mode".to_string());
    let mim2 = MinorMode::new("spell-check-mdoe".to_string());

    mim.push(mim1);
    mim.push(mim2);

    let mut buf = Buffer::new();
    buf.major_mode = mam;
    buf.minor_modes = mim;

    ws.buffers.push(buf);

    println!("{:#?}", ws);
}
