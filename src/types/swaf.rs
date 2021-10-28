////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
pub struct Swaf {
    pub version: u32,
    #[br(if(version >= 5))]
    lifetime_want_count: Option<u32>,
    #[br(if(version >= 5))]
    #[br(count = lifetime_want_count.unwrap() as usize)]
    pub lifetime_wants: Option<Vec<WantRecord>>,
    #[br(if(version >= 5, 4))]
    #[bw(map = |x: &u32| -> Option<u32> { if *x == 4 { None } else { Some(*x) } })]
    pub max_wants: u32,
    want_count: u32,
    #[br(count = want_count as usize)]
    pub wants: Vec<WantRecord>,
    #[br(if(version >= 5, 4))]
    #[bw(map = |x: &u32| -> Option<u32> { if *x == 4 { None } else { Some(*x) } })]
    max_fears: u32,
    fear_count: u32,
    #[br(count = fear_count as usize)]
    fears: Vec<WantRecord>,
    #[br(if(version >= 5))]
    unknown_1: Option<u32>,
    unknown_2: u32,
    counter: u32,
    previous_count: u32,
    #[br(count = previous_count as usize)]
    previous_wants_fears: Vec<PreviousWantsFears>,
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
struct PreviousWantsFears {
    id: u32,
    count: u32,
    #[br(count = count as usize)]
    records: Vec<WantRecord>,
}

#[binrw]
#[derive(Debug, PartialEq)]
struct WantRecord {
    version: u32,
    sim_instance_id: u16,
    want_id: u32,
    want_type: WantType,
    price: u16,
    counter: u32,
    aspiration: i32,
    #[br(if(version >= 9))]
    influence: Option<i32>,
    flags: u8,
}

#[binrw]
#[derive(Debug, PartialEq)]
pub enum WantType {
    #[brw(magic(0x00u8))]
    None,
    #[brw(magic(0x01u8))]
    Sim(u16),
    #[brw(magic(0x02u8))]
    Object(u32),
    #[brw(magic(0x03u8))]
    Category(u32),
    #[brw(magic(0x04u8))]
    Skill(u16),
    #[brw(magic(0x05u8))]
    Career(u32),
}
