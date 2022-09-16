////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::types::util::bytes::{Position, Size};
use binrw::binrw;

#[binrw]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[brw(little, magic = b"DBPF")]
pub struct Header {
    #[br(assert(major_version <= 1, "Not a Sims 2 DBPF File"))]
    pub major_version: u32,
    pub minor_version: u32,
    // There's five u32 fields skipped here
    // unknown possibly flags field
    // 2 fields unused in sims 2
    // and 2 date fields which aren't consistently used even in maxis material
    // Discarding them saves some memory and parse time
    #[brw(pad_before = 20)]
    #[br(assert(index_major_version == 7, "Not a Sims 2 DBPF File"))]
    pub index_major_version: u32,
    pub index_entry_count: u32,
    pub index_position: Position,
    pub index_size: Size,
    // next gap is the "hole" registry
    // used as a sort of trash when it is, but largely unused
    // Doesn't really matter, so safe to discard
    // 3 u32 entries
    #[brw(pad_before = 12)]
    // After reading the minor version, there's a large gap following the last element. Unused,
    // probably reserved
    #[brw(pad_after = 32)]
    #[br(if(minor_version >= 1))]
    pub index_minor_version: Option<u32>,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            major_version: 1,
            minor_version: 0,
            index_major_version: 7,
            index_entry_count: 0,
            index_position: Default::default(),
            index_size: Default::default(),
            index_minor_version: None,
        }
    }
}

impl Header {
    #[must_use]
    pub fn has_resource_id(&self) -> bool {
        self.index_minor_version.unwrap_or(0) >= 2
    }
}
