////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::constants::data_kinds::{DbpfKind, Id};
use crate::types::unimplemented::Unimplemented;
use binrw::binrw;

pub type POOL = PoolSurface;

pub type PoolSurface = Unimplemented;

/* TODO
#[binrw]
pub struct PoolSurface {}

impl DbpfKind for PoolSurface {
    fn id(&self) -> Id {
        Id::PoolSurface
    }
}
 */
