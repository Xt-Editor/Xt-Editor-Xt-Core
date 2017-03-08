//! Models for JSONI (JSON Interface)

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct XtensisObject {
    version: String,
    direction: String,
    comm_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MinorModeObject {
    id: String,
    active: bool,
    docstring: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BufferObject {
    uuid: String,
    file_path: String,
    active: bool,
    temporary: bool,
    major_mode: String,
    minor_modes: Vec<MinorModeObject>,
    read_only: bool,
    dirty: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BufferResponse {
    xtensis: XtensisObject,
    buffer: BufferObject,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CommandObject {
    uuid: String,
    name: String,
    docstring: String,
    cmd_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CommandResponse {
    xtensis: XtensisObject,
    command: CommandObject,
}
