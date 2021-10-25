////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;
use crate::constants::data_kinds::FormatKind;

#[binrw]
#[derive(Debug, PartialEq)]
struct Dir {
    entries: Vec<DirEntry>
}

#[binrw]
#[derive(Debug, PartialEq)]
struct DirEntry {
    type_id: FormatKind,
    group_id: u32,
    instance_id_lo: u32,
    instance_id_hi: u32,
    decompressed_size: u32,
}
