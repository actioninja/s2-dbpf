////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::constants::data_kinds::{DbpfEntry, DbpfId};
use crate::types::util::parser_args::ParserArgs;
use binrw::{binrw, NullString};
#[cfg(test)]
use test_strategy::Arbitrary;

pub type TPRP = BehaviorFunctionLabels;

#[binrw]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
#[br(import_raw(_args: ParserArgs))]
pub struct BehaviorFunctionLabels {
    #[br(try_map(NullString::try_into))]
    #[bw(map(| x: & String | NullString::from(x.clone())))]
    #[brw(pad_size_to = 64)]
    //Supposedly unused
    file_name: String,
}

impl DbpfEntry for BehaviorFunctionLabels {
    fn id(&self) -> DbpfId {
        DbpfId::BehaviorFunctionLabels
    }
}
