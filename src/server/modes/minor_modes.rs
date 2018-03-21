/// Minor mode struct for Xt.
/// TODO.
#[derive(Debug)]
pub struct MinorMode {
    /// Human-readable name for major mode.
    pub mim_hname: String,
}

impl MinorMode {
    /// Create a new instance of a Minor Mode.
    pub fn new(mim_hname: String) -> MinorMode {
        MinorMode {
            mim_hname,
        }
    }
}
