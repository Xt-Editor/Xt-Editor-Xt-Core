//! Models for JSONI (JSON Interface)

// This is the Xtensis text editor; it edits text.
// Copyright (C) 2016-2017  {The Xtensis Developers}

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
