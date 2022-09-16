////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::constants::data_kinds::{DbpfEntry, DbpfId};
use crate::types::util::parser_args::ParserArgs;
use binrw::binrw;
use derive_more::Constructor;

#[binrw]
#[derive(Debug, PartialEq, Eq, Clone, Constructor)]
#[br(import_raw(args: ParserArgs))]
pub struct Unimplemented {
    #[br(count(args.index_entry.size.0))]
    data: Vec<u8>,
}

impl DbpfEntry for Unimplemented {
    fn id(&self) -> DbpfId {
        DbpfId::UiData
    }
}
