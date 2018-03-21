/// Major Mode struct for Xt.
/// TODO.
#[derive(Debug)]
pub struct MajorMode {
    /// Human-readable name for major mode.
    pub mam_hname: String,
}

impl MajorMode {
    /// Create a new instance of a Minor Mode.
    pub fn new(mam_hname: String) -> MajorMode {
        MajorMode {
            mam_hname
        }
    }
}
