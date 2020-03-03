extern crate xt_core_system as xtcs;

#[macro_use]
extern crate slog;

use xtcs::logging::init_logger;

pub fn init_rpc() {
    let log = init_logger("xt_core_rpc");

    debug!(log,
        "RPC module initalising...";
        o!("module" => "xt.core.rpc"));
}
