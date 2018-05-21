/// Major Mode struct for Xt.
/// TODO.
#[derive(Debug)]
pub struct MajorMode {
    /// Human-readable name for major mode.
    pub hname: String,
}

impl MajorMode {
    /// Create a new instance of a Minor Mode.
    pub fn new(hname: String) -> MajorMode {
        MajorMode { hname }
    }
}
