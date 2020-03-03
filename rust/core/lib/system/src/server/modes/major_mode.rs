//! Major Mode struct for Xt.

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

/// Struct for a 'major mode', associated with a `Buffer` struct.
#[derive(Debug)]
pub struct MajorMode {
    /// Human-readable name for major mode.
    pub human_name: String,
}

impl MajorMode {
    /// Create a new instance of a Minor Mode.
    pub fn new(hname: String) -> MajorMode {
        MajorMode {
            human_name: human_name,
            owner: "shymega",
        }
    }
}
