//! This is the Server module for Xt.

extern crate gapbuffer;
extern crate uuid;

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
