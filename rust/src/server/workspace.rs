// //! Workspace module for Xt.
// //! Handles workspace and their associated buffers.

extern crate slog;

use logging::init_logger;
use server::buffer::Buffer;

/// Workspace struct
#[derive(Debug)]
pub struct Workspace {
    /// Human-readable name for workspace.
    pub hname: String,
    /// Logger instance for workspace.
    /// Derived from root Logger.
    pub logger: slog::Logger,
    /// Vector of Buffer structs.
    pub buffers: Vec<Buffer>,
}

impl Workspace {
    ///
    pub fn new(hname: String) -> Workspace {
        let logger = init_logger().new(o!("workspace" => hname.clone()));
        Workspace {
            hname: hname,
            logger: logger,
            buffers: Vec::new(),
        }
    }
}
