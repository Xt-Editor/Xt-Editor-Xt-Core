// //! Workspace module for Xt.
// //! Handles workspace and their associated buffers.

extern crate slog;

use logging::init_logger;

/// Workspace struct
#[derive(Debug)]
pub struct Workspace {
    /// Human-readable name for workspace.
    pub w_hname: String,
    /// Logger instance for workspace.
    /// Derived from root Logger.
    pub logger: slog::Logger,
}

impl Workspace {
    ///
    pub fn new(w_hname: String) -> Workspace {
        let w_logger = init_logger().new(o!("workspace" => w_hname.clone()));
        Workspace {
            w_hname,
            logger: w_logger,
        }
    }
}
