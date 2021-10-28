////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
struct Trcn {
    file_name: NullString,
    #[brw(pad_size_to = 64)]
    #[brw(magic(b"NCRT"))]
    unknown: u32,
    #[brw(pad_before = 32)]
    num_labels: u32,
    #[br(count(num_labels as usize))]
    labels: Vec<BconLabel>,
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
struct BconLabel {
    #[brw(pad_before = 32)]
    id_number: u32,
    name_length: u8,
    #[br(count(name_length as usize), try_map = String::from_utf8)]
    #[bw(map = |x: &String| x.as_bytes())]
    name: String,
    default_value: i16,
    min_value: i16,
    #[brw(pad_after = 9)]
    max_value: i16,
}
