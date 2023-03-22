////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::{binrw, NullString};

#[binrw]
#[derive(Default, Clone, PartialEq)]
pub struct GlobalData {
    #[br(try_map(NullString::try_into))]
    #[bw(map(| x: & String | NullString::from(x.clone())))]
    #[brw(pad_size_to = 64)]
    pub file_name: String,
}
