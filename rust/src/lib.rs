//! This is the main lib.rs file for Xt Core

// This file is part of Xt.

// This is the Xt text editor; it edits text.
// Copyright (C) 2016-2017  The Xt Developers

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

#![cfg_attr(feature = "ci", feature(plugin))]
#![cfg_attr(feature = "ci", plugin(clippy))]
#![cfg_attr(feature = "ci", allow(unstable_features))]
#![deny(
    missing_docs, missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts, unused_import_braces,
    unused_qualifications
)]

#[macro_use]
extern crate slog;

extern crate slog_async;
extern crate slog_term;

extern crate serde;

pub mod logging;
pub mod server;
pub mod utils;
