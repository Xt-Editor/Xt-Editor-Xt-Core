//! Major Mode struct for Xt.

/// Struct for a 'major mode', associated with a `Buffer` struct.
#[derive(Debug)]
pub struct MajorMode {
    /// Human-readable name for major mode.
    pub human_name: String,
}

impl Default for MajorMode {
    fn default() -> MajorMode {
        MajorMode {
            human_name: "fundamental-mode".to_owned(),
        }
    }
}

impl MajorMode {
    /// Create a new instance of a Minor Mode.
    pub fn new(human_name: String) -> MajorMode {
        MajorMode {
            human_name,
        }
    }
}
