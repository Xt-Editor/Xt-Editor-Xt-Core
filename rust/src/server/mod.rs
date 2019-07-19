//! This is the Server module for Xt.

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

extern crate gapbuffer;

/// `Buffer` struct module for Xt.
///
/// Each struct instance holds a Buffer's state, and links with the
/// associated workspace of the buffer.
/// If no workspace is defined, it uses the wildcard workspace.
pub mod buffer;

/// Workspace module for Xt.
/// Holds a collection of `Buffer` structs together with
/// `Vec<Buffers>`.
pub mod workspace;

/// Major and Minor modes for Xt.
/// Defines the basic structure for each modes, with (planned)
/// extensible.
pub mod modes;
