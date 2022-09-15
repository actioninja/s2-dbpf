////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;
use derive_more::{AsRef, Constructor, Display};
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use crate::constants::data_kinds::Id;
use crate::types::package::directory::{Dir, SIZE_OF_DIR_ENTRY, SIZE_OF_DIR_ENTRY_WITH_RESOURCE};
use crate::types::package::header::Header;
use crate::types::package::index_table::IndexTable;
use crate::types::util::bytes::{Position, Size};
#[cfg(test)]
use proptest::prelude::*;
use refpack::decompress;
#[cfg(test)]
use test_strategy::Arbitrary;

#[derive(Debug, Clone)]
pub struct Dbpf {
    header: Header,
    pub entries: HashMap<Key, Entry>,
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
        reader.seek(SeekFrom::Start(header.index_position.0 as u64))?;
        let has_resource = header.has_resource_id();
        let index_table =
            IndexTable::read_options(reader, options, (has_resource, header.index_entry_count))?;
        let compression_position = index_table
            .table
            .iter()
            .find(|(key, _entry)| key.kind == Id::Directory)
            .map(|(_, entry)| entry);

        let compression_table = if let Some(compression_entry) = compression_position {
            reader.seek(SeekFrom::Start(compression_entry.location.0 as u64))?;
            let entry_size = if header.has_resource_id() {
                SIZE_OF_DIR_ENTRY_WITH_RESOURCE
            } else {
                SIZE_OF_DIR_ENTRY
            };
            let entry_count = compression_entry.size.0 / entry_size.0;
            let dir = Dir::read_options(reader, options, (header.has_resource_id(), entry_count))?;
            Some(dir)
        } else {
            None
        };
        reader.seek(SeekFrom::Start(store_pos))?;

        let mut entries_table = HashMap::new();
        for (key, entry) in index_table.table {
            let new_kind = if let Some(ref compression_table) = compression_table {
                let decompressed_size = compression_table.table[&key].decompressed_size.0;
                let mut decomp_buffer = Cursor::new(vec![0u8; decompressed_size as usize]);
                decompress(reader, &mut decomp_buffer).expect("Decompression failed");
                decomp_buffer.set_position(0);
                ::parse(&mut decomp_buffer, key.kind, options, entry.size)?
            } else {
                Kind::parse(reader, key.kind, options, entry.size)?
            };
            let new_entry = Entry { compressed: false };
            entries_table.insert(key, new_entry);
        }

        Ok(Dbpf {
            header,
            entries: entries_table,
        })
    }
}

/*
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
 */

#[binrw]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Display, Hash, Default, Constructor)]
#[display(
    fmt = "{}-{}-{}-{}",
    "kind.short_name()",
    group_id,
    instance_id,
    "resource_id.unwrap_or(ResourceId(0))"
)]
#[br(import(has_resource: bool))]
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, arbitrary(args = (bool,)))]
pub struct Key {
    pub kind: Id,
    pub group_id: GroupId,
    pub instance_id: InstanceId,
    #[br(if(has_resource))]
    #[cfg_attr(test, strategy(any::<ResourceId>().prop_map(move |x| if args.0 { Some(x)} else { None })))]
    pub resource_id: Option<ResourceId>,
}

#[binrw]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, Default)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct GroupId(pub u32);

#[binrw]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, Default)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct InstanceId(pub u32);

#[binrw]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, Default)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ResourceId(pub u32);

#[derive(Debug, Clone)]
pub struct Entry {
    compressed: bool,
    //data:
}
