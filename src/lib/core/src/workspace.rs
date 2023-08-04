//! Workspace module for Xt.
//! Handles workspaces and their associated buffers.

use crate::buffer::Buffer;
use uuid::Uuid;

/// Workspace struct
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Human-readable name for workspace.
    human_name: String,
    /// Workspace UUID
    uuid: Uuid,
    /// Vector of Buffer structs.
    buffers: Vec<Buffer>,
}

impl Workspace {
    /// Create a new workspace.
    pub fn new(human_name: &str) -> Self {
        Self {
            human_name: String::from(human_name),
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
        self.uuid.as_hyphenated().to_string()
    }

    /// Get 'owned' `Vec<Buffer>`.
    pub fn get_buffers(&self) -> Vec<Buffer> {
        self.buffers.clone()
    }
}
