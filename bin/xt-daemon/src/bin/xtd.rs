//! This module serves as the entry point into Xt's main binary.

extern crate clap;

extern crate xt_core_daemon as xt_daemon;
extern crate xt_core_system as xt_system;
extern crate xt_core_utils as xt_utils;

#[macro_use]
extern crate slog;
extern crate slog_term;

use clap::{App, Arg, ArgMatches, SubCommand};
use xt_utils::init_logger;

fn retrieve_arguments() -> ArgMatches<'static> {
    App::new("xt-core")
        .version("0.1.0")
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("Core backend for Xt.")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .required(false),
        )
        .arg(Arg::with_name("debug"))
        .subcommand(
            SubCommand::with_name("spawn").help("Spawn a instance of Xt"),
        )
        .get_matches()
}

fn main() {
    let args = retrieve_arguments();

    let log_level = match args.occurrences_of("verbose") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        _ => slog::Level::Trace,
    };

    let log = init_logger("xt_daemon", log_level);

    info!(log, "Xt (core) loading..";
        o!("stage" => "init"));

    debug!(log,
        "Checking for configuration file..";
        o!("stage" => "init.load.conf"));

    debug!(log,
        "Found configuration file!";
        o!("stage" => "init.load.conf"));

    warn!(
        log,
        "Xt (core) has no configuration file. Reverting to defaults.";
        o!("stage" => "init.load.conf"));

    info!(log,
        "Configuration loaded, initialising...";
        o!("stage" => "init.load.conf"));

    error!(log, "Xt Core is not ready for deployment. Halt.";
        o!("stage" => "init.finalise.halt"));

    unimplemented!();
}
