//! Major Mode struct for Xt.

/// Struct for a 'major mode', associated with a `Buffer` struct.
#[derive(Debug, Clone)]
pub struct MajorMode {
    /// Human-readable name for major mode.
    pub human_name: String,
}

impl Default for MajorMode {
    fn default() -> Self {
        Self {
            human_name: String::from("fundamental-mode"),
        }
    }
}

impl MajorMode {
    /// Create a new instance of a Minor Mode.
    pub fn new(human_name: String) -> Self {
        Self { human_name }
    }
}
