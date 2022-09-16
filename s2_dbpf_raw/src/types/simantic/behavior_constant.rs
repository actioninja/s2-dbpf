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

pub type BCON = BehaviorConstants;

#[binrw]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
#[brw(little)]
#[br(import_raw(args: ParserArgs))]
pub struct BehaviorConstants {
    #[br(try_map(NullString::try_into))]
    #[bw(map(| x: & String | NullString::from(x.clone())))]
    #[brw(pad_size_to = 64)]
    pub file_name: String,
    #[br(temp)]
    #[bw(calc = constants.len() as u8)]
    count: u8,
    //Unknown
    pub flags: u8,
    #[br(count(count))]
    pub constants: Vec<i16>,
}

impl DbpfEntry for BehaviorConstants {
    fn id(&self) -> DbpfId {
        DbpfId::BehaviorConstant
    }

    fn name(&self) -> Option<String> {
        Some(self.file_name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use binrw::*;
    use proptest::prelude::*;
    use std::io::Cursor;
    use test_strategy::*;

    test_parsing!(
        [
            0x54, 0x65, 0x73, 0x74, 0x46, 0x69, 0x6C, 0x65, //TestFile - in ascii bytes
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // first padding block
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // second padding block
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // third padding block
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // fourth padding block
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // fifth padding block
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // sixth padding block
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // seventh padding block
            0x08, // count
            0x08, // flags
            0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, // constants
            0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00, // constants
        ],
        BehaviorConstants {
            file_name: "TestFile".to_string(),
            flags: 8,
            constants: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        BehaviorConstants,
        bcon
    );
}
