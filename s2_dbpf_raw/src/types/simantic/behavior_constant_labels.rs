////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::constants::data_kinds::{DbpfKind, Id};
use binrw::{binrw, NullString};

pub type TRCN = BehaviorConstantLabels;

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
pub struct BehaviorConstantLabels {
    #[br(map(NullString::into_string))]
    #[bw(map(| x: & String | NullString::from_string(x.clone())))]
    pub file_name: String,
    #[brw(pad_size_to = 64)]
    #[brw(magic(b"NCRT"))]
    pub unknown: u32,
    #[brw(pad_before = 32)]
    #[br(temp)]
    #[bw(calc = labels.len() as u32)]
    num_labels: u32,
    #[br(count(num_labels as usize))]
    pub labels: Vec<BconLabel>,
}

impl DbpfKind for BehaviorConstantLabels {
    fn id(&self) -> Id {
        Id::BehaviorFunctionLabels
    }

    fn name(&self) -> Option<String> {
        Some(self.file_name.clone())
    }
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
pub struct BconLabel {
    #[brw(pad_before = 32)]
    pub id_number: u32,
    #[br(temp)]
    #[bw(calc = name.len() as u8)]
    name_length: u8,
    #[br(count(name_length as usize), try_map = String::from_utf8)]
    #[bw(map = std::string::String::as_bytes)]
    pub name: String,
    pub default_value: i16,
    pub min_value: i16,
    #[brw(pad_after = 9)]
    pub max_value: i16,
}
