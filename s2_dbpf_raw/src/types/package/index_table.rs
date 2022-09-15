////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::{binrw, BinRead, BinResult, BinWrite, ReadOptions, WriteOptions};

use crate::types::package::database_packed_file::Key;
use crate::types::util::bytes::{Position, Size};
use std::collections::HashMap;
use std::io::{Read, Seek, Write};

pub struct IndexTable {
    pub table: HashMap<Key, Entry>,
}

impl BinRead for IndexTable {
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

        Ok(IndexTable { table })
    }
}

impl BinWrite for IndexTable {
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Entry {
    pub location: Position,
    pub size: Size,
}
