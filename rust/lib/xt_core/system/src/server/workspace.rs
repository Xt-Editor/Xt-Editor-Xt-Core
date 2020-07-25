//! Workspace module for Xt.
//! Handles workspaces and their associated buffers.

// This file is part of Xt.

// This is the Xt text editor; it edits text.
// Copyright (C) 2016-2018  The Xt Developers

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

extern crate slog;
extern crate uuid;

use self::uuid::Uuid;
use logging::init_logger;
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
        let logger = init_logger("workspace", slog::Level::Debug).new(o!("workspace" => human_name.clone()));

        Workspace {
            human_name: human_name,
            logger: logger,
            uuid: Uuid::new_v4(),
            buffers: Vec::new(),
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_fmt_uuid(&self) -> String {
        self.uuid.to_hyphenated().to_owned().to_string()
    }
}
