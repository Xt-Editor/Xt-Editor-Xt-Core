//! This is the Buffer module for Xt.

// This file is part of Xt.
//
// This is the Xt text editor; it edits text.
// Copyright (C) 2016-2018 The Xt Developers

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

use self::gapbuffer::GapBuffer;
use std::path::PathBuf;

use server::modes::major_mode::MajorMode;
use server::modes::minor_mode::MinorMode;

/// Struct for a Buffer object in Xt.
/// Stores buffer state & metadata.
#[derive(Debug)]
pub struct Buffer {
    /// File path of a buffer.
    pub file_path: Option<PathBuf>,
    /// Active status of a buffer.
    pub active: bool,
    /// Temporary status of a buffer.
    pub temporary: bool,
    /// If a buffer is r/o.
    /// If this is `true`, then r/w is `false.`
    /// Likewise, if this is `false`, then r/w is `true`.
    pub read_only: bool,
    /// Major mode of a buffer
    pub major_mode: MajorMode,
    /// Array of Minor modes in a buffer
    pub minor_modes: Vec<MinorMode>,
    /// Dirty status of a buffer
    pub dirty: bool,
    /// Contents of a buffer.
    pub text: GapBuffer<u8>,
}

impl Buffer {
    /// Return a new instance of `Buffer`.
    pub fn new() -> Buffer {
        Buffer {
            file_path: None,
            active: false,
            temporary: false,
            read_only: false,
            major_mode: MajorMode::new("fundamental-mode".to_owned()),
            minor_modes: Vec::new(),
            dirty: false,
            text: GapBuffer::new(),
        }
    }

    /// Return true if a buffer is active.
    /// That is to say, a 'focused' buffer, or otherwise in-use, and
    /// *not* inactive.
    /// If it's not a active buffer, then return false.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Return true if a buffer is temporary.
    /// If it's not a temporary buffer, then return false.
    /// A temporary buffer is defined by the characteristic of having
    /// no 'target' to save to.
    pub fn is_temporary(&self) -> bool {
        self.temporary
    }

    /// Return true if a buffer is read only.

    /// If it's not read-only, then it returns false, and the buffer
    /// will be presumed to be read-write.

    /// A read-only buffer is defined by the characteristic of the FS
    /// denying write access, or being set manually.
    pub fn is_ro(&self) -> bool {
        self.read_only
    }

    /// Return true if a buffer is dirty.
    /// If it's not dirty, then return false.
    /// A dirty buffer is defined by the characteristic of the buffer
    /// having changed since the last save.
    /// Temporary buffers are *exempt* from this field.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Return the current major mode of a buffer.
    pub fn get_major_mode(&self) -> &MajorMode {
        &self.major_mode
    }

    /// Return a `Vec<&'static str>` of minor modes.
    pub fn get_minor_modes(&self) -> &Vec<MinorMode> {
        &self.minor_modes
    }

    /// Return the length of a buffer.
    pub fn get_buffer_length(&self) -> usize {
        self.text.len()
    }
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer::new()
    }
}

#[cfg(test)]
mod test {
    use super::Buffer;
    use super::{MajorMode, MinorMode};

    #[test]
    fn test_buffer_default_values() {
        let buf = Buffer::new();

        assert_eq!(None, buf.file_path);
        assert_eq!(false, buf.active);
        assert_eq!(false, buf.is_active());

        assert_eq!(false, buf.temporary);
        assert_eq!(false, buf.is_temporary());

        assert_eq!(false, buf.read_only);
        assert_eq!(false, buf.is_ro());

        assert_eq!("fundamental-mode", buf.major_mode.hname);
        assert_eq!(false, buf.dirty);
    }

    #[test]
    fn test_buffer_major_minor_modes() {
        let mut buf = Buffer::new();
        let mam = MajorMode::new("rust-mode".to_owned());
        let mut mim: Vec<MinorMode> = Vec::new();

        buf.major_mode = mam;
        mim.push(MinorMode::new("auto-wrap".to_owned()));
    }
}
