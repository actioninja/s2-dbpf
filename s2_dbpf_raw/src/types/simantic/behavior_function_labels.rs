////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::constants::data_kinds::{DbpfKind, Id};
use binrw::{binrw, NullString};
#[cfg(test)]
use test_strategy::Arbitrary;

pub type TPRP = BehaviorFunctionLabels;

#[binrw]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BehaviorFunctionLabels {
    #[br(map(NullString::into_string))]
    #[bw(map(| x: & String | NullString::from_string(x.clone())))]
    #[brw(pad_size_to = 64)]
    //Supposedly unused
    file_name: String,
}

impl DbpfKind for BehaviorFunctionLabels {
    fn id(&self) -> Id {
        Id::BehaviorFunctionLabels
    }
}
