////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::types::package::header::Header;
use crate::types::package::index_table::Entry;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ParserArgs {
    pub header: Header,
    pub index_entry: Entry,
}
