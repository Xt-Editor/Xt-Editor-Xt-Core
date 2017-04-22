//! This is the main lib.rs file for Xtensis-core

// This file is part of Xtensis.

// This is the Xtensis text editor; it edits text.
// Copyright (C) 2016-2017  The Xtensis Developers

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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![deny(missing_docs, missing_debug_implementations,
        missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unused_import_braces,
        unused_qualifications, clippy)]

#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_async;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod logging;
pub mod rpc;
pub mod server;
pub mod utils;
