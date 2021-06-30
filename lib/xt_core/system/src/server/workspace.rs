//! Workspace module for Xt.
//! Handles workspaces and their associated buffers.

use super::uuid::Uuid;

use crate::utils::init_logger;

use server::buffer::Buffer;

/// Workspace struct
#[derive(Debug)]
pub struct Workspace {
    /// Human-readable name for workspace.
    pub human_name: String,
    /// Workspace UUID
    pub uuid: Uuid,
    /// Logger instance for workspace.
    pub logger: slog::Logger,
    /// Vector of Buffer structs.
    pub buffers: Vec<Buffer>,
}

impl Workspace {
    /// Create a new workspace.
    pub fn new(human_name: String) -> Workspace {
        let logger = init_logger("workspace", slog::Level::Debug)
            .new(o!("workspace" => human_name.clone()));

        Workspace {
            human_name,
            logger,
            uuid: Uuid::new_v4(),
            buffers: Vec::new(),
        }
    }

    /// Get UUID of workspace.
    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    /// Get (formatted, hyphen) UUID of workspace.
    pub fn get_fmt_uuid(&self) -> String {
        self.uuid.to_hyphenated().to_owned().to_string()
    }
}
