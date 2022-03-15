////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

//! Directory of compressed files within the dbpf file
//! Used internally to track if files are compressed before accessing them.

use binrw::binrw;
#[cfg(test)]
use proptest::collection::vec;
#[cfg(test)]
use proptest::num::u32::ANY;
#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use test_strategy::Arbitrary;

use crate::constants::data_kinds::FormatId;

const SIZE_OF_DIR_ENTRY: u32 = 128; // 4 DWORDS
const SIZE_OF_DIR_ENTRY_HI: u32 = SIZE_OF_DIR_ENTRY + 32; // extra hi entry bumps up the length by one more DWORD

#[derive(Debug, PartialEq)]
#[binrw]
#[br(import(size_of_file: u32, hi_instance: bool))]
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, arbitrary(args = (bool,)))]
pub struct Dir {
    #[br(args{count: (size_of_file / (if hi_instance { SIZE_OF_DIR_ENTRY_HI } else { SIZE_OF_DIR_ENTRY })) as usize, inner: (hi_instance,)})]
    #[cfg_attr(test, strategy(vec(any_with::<DirEntry>((args.0,)), 0..=100)))]
    entries: Vec<DirEntry>,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
#[binrw]
#[br(import(hi_instance: bool))]
#[cfg_attr(test, derive(Arbitrary))]
#[cfg_attr(test, arbitrary(args = (bool,)))]
struct DirEntry {
    type_id: FormatId,
    group_id: u32,
    instance_id_lo: u32,
    #[br(if(hi_instance))]
    #[cfg_attr(test, strategy(ANY.prop_map(move |x| if args.0 { Some(x)} else { None })))]
    instance_id_hi: Option<u32>,
    decompressed_size: u32,
}

#[cfg(test)]
mod test {
    use super::*;
    use binrw::{BinReaderExt, BinWriterExt};
    use std::io::Cursor;
    use test_strategy::proptest;
    use test_strategy::*;

    #[proptest]
    fn symmetrical_write(has_hi: bool, #[any((#has_hi,))] in_dir: Dir) {
        let mut writer = Cursor::new(vec![]);
        let num_files = in_dir.entries.len();
        let out_size = num_files as u32
            * if has_hi {
                SIZE_OF_DIR_ENTRY_HI
            } else {
                SIZE_OF_DIR_ENTRY
            };
        writer.write_le(&in_dir).unwrap();

        writer.set_position(0);

        let out: Dir = writer.read_le_args((num_files as u32, has_hi)).unwrap();

        prop_assert_eq!(out, in_dir);
    }
}
