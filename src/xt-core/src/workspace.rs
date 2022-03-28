//! Workspace module for Xt.
//! Handles workspaces and their associated buffers.

use crate::buffer::Buffer;
use uuid::Uuid;

/// Workspace struct
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Human-readable name for workspace.
    pub human_name: String,
    /// Workspace UUID
    pub uuid: Uuid,
    /// Vector of Buffer structs.
    pub buffers: Vec<Buffer>,
}

impl Workspace {
    /// Create a new workspace.
    pub fn new(human_name: String) -> Self {
        Self {
            human_name,
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
