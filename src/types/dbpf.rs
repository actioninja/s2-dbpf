////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::compression::CompressedFile;
use crate::constants::data_kinds::FormatKind;
use binrw::*;
use std::io::SeekFrom;

#[binrw]
#[derive(Debug, PartialEq)]
struct Dbpf {
    header: DbpfHeader,
    #[brw(seek_before(SeekFrom::Start(header.index_location as u64)))]
    #[br(args { count: header.index_entry_count as usize, inner: (header.index_minor_version == 2,) })]
    index_table: Vec<DbpfIndexEntry>,
    //#[brw(restore_position)]
    //data: Vec<u8>,
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
    index_location: u32,
    index_size: u32,
    hole_entry_count: u32,
    hole_offset: u32,
    hole_size: u32,
    #[brw(pad_after = 32)]
    index_minor_version: u32,
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
#[br(import(has_hi: bool))]
struct DbpfIndexEntry {
    kind: FormatKind,
    group_id: u32,
    instance_id_lo: u32,
    #[br(if(has_hi))]
    instance_id_hi: Option<u32>,
    location_in_archive: u32,
    size_in_archive: u32,
}
