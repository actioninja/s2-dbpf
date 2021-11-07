////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::constants::data_kinds::FormatKind;
use binrw::binrw;

#[binrw]
#[derive(Debug, PartialEq)]
#[br(import(number_of_files: u32))]
pub struct Dir {
    #[br(count = number_of_files)]
    entries: Vec<DirEntry>,
}

#[binrw]
#[derive(Debug, PartialEq)]
struct DirEntry {
    type_id: FormatKind,
    group_id: u32,
    instance_id_lo: u32,
    instance_id_hi: Option<u32>,
    decompressed_size: u32,
}
