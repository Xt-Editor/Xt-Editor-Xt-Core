//! This is the Buffer module for Xt.

use crate::modes::major_mode::MajorMode;
use crate::modes::minor_mode::MinorMode;
use gapbuffer::GapBuffer;
use std::path::PathBuf;

/// Struct for a Buffer object in Xt.
/// Stores buffer state & metadata.
#[derive(Debug, Clone)]
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
    pub major_mode: Option<MajorMode>,
    /// Array of Minor modes in a buffer
    pub minor_modes: Option<Vec<MinorMode>>,
    /// Dirty status of a buffer
    pub dirty: bool,
    /// Contents of a buffer.
    pub text: Option<GapBuffer<u8>>,
}

impl Default for Buffer {
    /// Return a new `Buffer`.
    fn default() -> Self {
        Self {
            file_path: None,
            active: false,
            temporary: false,
            read_only: false,
            major_mode: Some(MajorMode::default()),
            minor_modes: Some(Vec::new()),
            dirty: false,
            text: Some(GapBuffer::new()),
        }
    }
}

impl Buffer {
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
        self.file_path.is_none()
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
    pub fn get_major_mode(&self) -> MajorMode {
        self.major_mode.as_ref().unwrap().clone()
    }

    /// Return a `Vec<MinorMode>` array of minor modes.
    pub fn get_minor_modes(&self) -> Vec<MinorMode> {
        self.minor_modes.as_ref().unwrap().to_vec()
    }

    /// Return the length of a buffer.
    pub fn get_buffer_length(&self) -> usize {
        self.text.as_ref().unwrap().len()
    }
}

#[cfg(test)]
mod test {
    use super::Buffer;

    #[test]
    fn test_default_values_buffer() {
        let buf = Buffer::default();

        assert_eq!(false, buf.active);
        assert_eq!(false, buf.is_active());

        assert_eq!(false, buf.temporary);
        assert_eq!(false, buf.is_temporary());

        assert_eq!(false, buf.read_only);
        assert_eq!(false, buf.is_ro());

        assert_eq!(0, buf.get_buffer_length());

        assert_eq!("fundamental-mode", buf.get_major_mode().human_name);

        assert_eq!(false, buf.dirty);

        assert_eq!(0, buf.text.unwrap().len());
    }
}
