////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::types::util::bytes::{Position, Size};
use binrw::binrw;

#[binrw]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[brw(little, magic = b"DBPF")]
pub struct Header {
    #[br(assert(major_version <= 1))]
    pub major_version: u32,
    pub minor_version: u32,
    pub major_user_version: u32,
    pub minor_user_version: u32,
    pub flags: u32,
    pub date_created: u32,
    pub date_modified: u32,
    pub index_major_version: u32,
    pub index_entry_count: u32,
    pub index_position: Position,
    pub index_size: Size,
    pub hole_entry_count: u32,
    pub hole_offset: u32,
    pub hole_size: Size,
    #[brw(pad_after = 32)]
    #[br(if(minor_version >= 1))]
    index_minor_version: Option<u32>,
}

impl Header {
    pub fn has_resource_id(&self) -> bool {
        self.index_minor_version.unwrap_or(0) >= 2
    }
}
