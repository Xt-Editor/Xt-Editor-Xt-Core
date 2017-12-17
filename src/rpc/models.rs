//! Models for JSONI (JSON Interface)

// This file is part of Xt.

// This is the Xt text editor; it edits text.
// Copyright (C) 2016-2017  The Xt Developers

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

/// JSON object for Xt RPC responses.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct XtObject {
    version: String,
    direction: String,
    comm_type: String,
}

/// JSON object for Minor modes.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MinorModeObject {
    id: String,
    active: bool,
    docstring: String,
}

/// JSON object for Buffer.
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

/// Buffer request response.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BufferResponse {
    xt: XtObject,
    buffer: BufferObject,
}

/// Command Object in JSON.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CommandObject {
    uuid: String,
    name: String,
    docstring: String,
    cmd_type: String,
}

/// Command response in JSON.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CommandResponse {
    xt: XtObject,
    command: CommandObject,
}
