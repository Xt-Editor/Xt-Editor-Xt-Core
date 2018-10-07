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
