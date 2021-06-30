//! Minor mode struct for Xt.

/// Struct for a `minor mode`, associated with a `Buffer` struct.
#[derive(Debug)]
pub struct MinorMode {
    /// Human-readable name for major mode.
    pub human_name: String,
}

impl MinorMode {
    /// Create a new instance of a Minor Mode.
    pub fn new(human_name: String) -> MinorMode {
        MinorMode {
            human_name
        }
    }
}
