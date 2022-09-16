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

//! Directory of compressed files within the dbpf file
//! Used internally to track if files are compressed before accessing them.

use binrw::{binrw, BinRead, BinResult, BinWrite, ReadOptions, WriteOptions};
#[cfg(test)]
use proptest::collection::hash_map;
#[cfg(test)]
use proptest::prelude::*;
use std::collections::HashMap;
use std::io::{Read, Seek, Write};
#[cfg(test)]
use test_strategy::Arbitrary;

use crate::types::package::database_packed_file::Key;
use crate::types::util::bytes::Size;

pub const SIZE_OF_DIR_ENTRY: Size = Size::dword(4);
pub const SIZE_OF_DIR_ENTRY_WITH_RESOURCE: Size = Size::dword(5); // extra hi entry bumps up the length by one more DWORD

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, arbitrary(args = (bool,)))]
pub struct Dir {
    #[cfg_attr(test, strategy(hash_map(any_with::<Key>((args.0,)), any::<Entry>(), (1..100))))]
    pub table: HashMap<Key, Entry>,
}

//TODO: See if I can turn this in to a binrw repr-based code reuse instead of duplicating this from the index table
impl BinRead for Dir {
    type Args = (bool, u32);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let (has_resource, entry_count) = args;

        let mut table: HashMap<Key, Entry> = HashMap::new();

        for _ in 0..entry_count {
            let key = Key::read_options(reader, options, (has_resource,))?;
            let entry = Entry::read_options(reader, options, ())?;

            table.insert(key, entry);
        }

        Ok(Dir { table })
    }
}

impl BinWrite for Dir {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> BinResult<()> {
        for (key, entry) in &self.table {
            Key::write_options(key, writer, options, ())?;
            Entry::write_options(entry, writer, options, ())?;
        }
        Ok(())
    }
}

#[binrw]
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd)]
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, arbitrary(args = (bool,)))]
pub struct Entry {
    pub decompressed_size: Size,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constants::data_kinds::DbpfId;
    use crate::types::package::database_packed_file::{GroupId, InstanceId, Key, ResourceId};
    use binrw::{BinReaderExt, BinWriterExt};
    use std::io::Cursor;
    use test_strategy::proptest;
    

    #[test]
    fn simple_test() {
        let mut in_dir = Dir {
            table: HashMap::new(),
        };
        let key = Key::new(
            DbpfId::UiData,
            GroupId(1),
            InstanceId(2),
            Some(ResourceId(3)),
        );
        let key2 = Key::new(
            DbpfId::UiData,
            GroupId(2),
            InstanceId(2),
            Some(ResourceId(3)),
        );
        let dir_entry = Entry {
            decompressed_size: Size(8),
        };
        in_dir.table.insert(key, dir_entry);
        in_dir.table.insert(key2, dir_entry);
        let mut writer = Cursor::new(vec![]);
        let num_files = in_dir.table.len();
        writer.write_le(&in_dir).unwrap();

        writer.set_position(0);

        let out: Dir = writer.read_le_args((true, num_files as u32)).unwrap();

        assert_eq!(in_dir, out);
    }

    #[proptest]
    fn symmetrical_write(has_resource: bool, #[any((#has_resource,))] in_dir: Dir) {
        let mut writer = Cursor::new(vec![]);
        let num_files = in_dir.table.len();
        writer.write_le(&in_dir).unwrap();

        let _clone = writer.clone().into_inner();

        //println!("Dir: {:?}", in_dir);

        //println!("Written vec: {:x?}", clone);

        writer.set_position(0);

        let out: Dir = writer
            .read_le_args((has_resource, num_files as u32))
            .unwrap();

        prop_assert_eq!(out, in_dir);
    }
}
