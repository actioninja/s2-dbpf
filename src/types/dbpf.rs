////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::constants::data_kinds::FormatKind;
use binrw::*;

#[binrw]
#[derive(Debug, PartialEq)]
struct Dbpf {
    header: DbpfHeader,
    #[brw(seek(), args(header.index_minor_version == 2))]
    index_table: DbpfIndexTable,
    data: Vec<u8>,
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little, magic = b"DBPF")]
struct DbpfHeader {
    major_version: u32,
    minor_version: u32,
    major_user_version: u32,
    minor_user_version: u32,
    flags: u32,
    date_created: u32,
    date_modified: u32,
    index_major_version: u32,
    index_entry_count: u32,
    first_index_entry_offset: u32,
    index_size: u32,
    hole_entry_count: u32,
    hole_offset: u32,
    hole_size: u32,
    #[brw(pad_after = 32)]
    index_minor_version: u32,
}


#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little, import(has_hi: bool))]
struct DbpfIndexTable {
    #[br(args(has_hi))]
    entries: Vec<DbpfIndexEntry>,
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little, import(has_hi: bool))]
struct DbpfIndexEntry {
    kind: FormatKind,
    group_id: u32,
    instance_id_lo: u32,
    #[br(if(has_hi))]
    instance_id_hi: Option<u32>,
    location_in_archive: u32,
    size_in_archive: u32,
}
