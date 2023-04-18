//! This is the Buffer module for Xt.

use crate::modes::MajorMode;
use crate::modes::MinorMode;
use ropey::Rope;
use std::path::PathBuf;

/// Struct for a Buffer object in Xt.
/// Stores buffer state & metadata.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// File path of a buffer.
    target: Option<PathBuf>,
    /// Active status of a buffer.
    active: bool,
    /// Temporary status of a buffer.
    temporary: bool,
    /// If a buffer is read-only.
    read_only: bool,
    /// Major mode of a buffer.
    major_mode: MajorMode,
    /// Array of Minor modes in a buffer.
    minor_modes: Vec<MinorMode>,
    /// Dirty status of a buffer.
    dirty: bool,
    /// Contents of a buffer.
    contents: Rope,
}

impl Default for Buffer {
    /// Return a new `Buffer`.
    fn default() -> Self {
        Self {
            target: None,
            active: false,
            temporary: false,
            read_only: false,
            major_mode: MajorMode::default(),
            minor_modes: Vec::new(),
            dirty: false,
            contents: Rope::new(),
        }
    }
}

impl Buffer {
    /// Return true if a buffer is active.
    ///
    /// That is to say, a 'focused' buffer, or otherwise in-use, and
    /// *not* inactive.
    ///
    /// If it's not a active buffer, then return false.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Return true if a buffer is temporary.
    /// If it's not a temporary buffer, then return false.
    /// A temporary buffer is defined by the characteristic of having
    /// no 'target' to save to.
    pub fn is_temporary(&self) -> bool {
        self.target.is_none()
    }

    /// Return true if a buffer is read only.
    ///
    /// Being read-only has a special meaning in Xt.
    ///
    /// It means when a buffer is explictly prevented from being
    /// written to, or otherwise manipulated.
    ///
    /// It has nothing to do with the underlying filesystem.
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Return true if a buffer is dirty.
    /// If it's not dirty, then return false.
    ///
    /// A dirty buffer is defined by the characteristic of the buffer having changed since the last
    /// save.
    ///
    /// Temporary buffers are *exempt* from this field, and return false when questioned.
    pub fn is_dirty(&self) -> bool {
        if self.temporary {
            false
        } else {
            self.dirty
        }
    }

    /// Return the current major mode of a buffer.
    pub fn get_major_mode(&self) -> Option<MajorMode> {
        Some(self.major_mode.clone())
    }

    /// Return a `Vec<MinorMode>` array of minor modes.
    pub fn get_minor_modes(&self) -> Option<Vec<MinorMode>> {
        Some(self.minor_modes.clone())
    }
}

#[cfg(test)]
mod test {
    use super::Buffer;

    #[test]
    fn test_default_values_buffer() {
        let buf = Buffer::default();

        assert_eq!(false, buf.is_active());

        assert_eq!(true, buf.is_temporary());

        assert_eq!(false, buf.is_read_only());

        assert_eq!(false, buf.is_dirty());

        assert_eq!(
            "fundamental-mode",
            buf.get_major_mode().unwrap().human_name
        );
    }
}
