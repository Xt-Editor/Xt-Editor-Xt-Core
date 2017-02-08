//! Models for JSONI (JSON Interface)

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct XtensisObject {
    version: String,
    direction: String,
    comm_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BufferObject {
    uuid: String,
    active: bool,
    temporary: bool,
    major_mode: String,
    minor_modes: Vec<String>,
    read_only: bool,
    dirty: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BufferResponse {
    xtensis: XtensisObject,
    buffer: BufferObject,
}
