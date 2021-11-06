////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::{binrw, NullString};

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
struct Bcon {
    file_name: NullString,
    #[brw(pad_size_to = 64)]
    count: u8,
    //Unknown
    flags: u8,
    #[br(count(count))]
    constants: Vec<i16>,
}
