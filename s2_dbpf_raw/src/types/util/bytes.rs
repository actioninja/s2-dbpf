////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::binrw;
use derive_more::{Add, Display, Sub};
#[cfg(test)]
use test_strategy::Arbitrary;

#[binrw]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Display, Hash, Default, Add, Sub)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Size(pub u32);

impl Size {
    #[must_use]
    pub const fn word(number: u32) -> Self {
        Size(number * 2)
    }

    #[must_use]
    pub const fn dword(number: u32) -> Self {
        Size(number * 4)
    }
}

#[binrw]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Display, Hash, Default)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Position(pub u32);
