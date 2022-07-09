////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::constants::data_kinds::Id;
use crate::types::directory::Dir;

#[derive(Debug, PartialEq)]
struct Dbpf {
    header: Header,
    index_table: Vec<IndexEntry>,
    compression_table: Dir,
    //#[brw(restore_position)]
    //data: Vec<u8>
}

impl BinRead for Dbpf {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let header = Header::read(reader)?;
        let store_pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(header.index_location as u64))?;
        let index_table: Vec<IndexEntry> = todo!();
        /*
        let compression_table = index_table
            .into_iter()
            .find(|entry| entry.kind == FormatKind::Compression)?;

        Ok(Dbpf {
            header,
            index_table,
            compression_table: (),
        })
         */
    }
}

impl BinWrite for Dbpf {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        args: Self::Args,
    ) -> BinResult<()> {
        todo!();

        Ok(())
    }
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little, magic = b"DBPF")]
struct Header {
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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct DbpfKey {
    kind: Id,
    group_id: GroupId,
    resource_id: ResourceId,
    instance_id: InstanceId,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct GroupId(pub u32);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct ResourceId(pub u32);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct InstanceId(pub u32);

//#[binrw]
#[derive(Debug, PartialEq)]
//#[brw(little)]
//#[br(import(has_hi: bool))]
struct IndexEntry {
    kind: Id,
    group_id: u32,
    instance_id_lo: u32,
    //#[br(if(has_hi))]
    instance_id_hi: Option<u32>,
    location_in_archive: u32,
    size_in_archive: u32,
    //#[brw(ignore)]
    compressed: bool,
}
