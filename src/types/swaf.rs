////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;
use proptest::prelude::*;
use test_strategy::Arbitrary;

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
pub struct Swaf {
    pub version: SwafVersion,
    #[br(if(version != SwafVersion::One), temp)]
    #[bw(calc = if let Some(lifetime_want_vec) = lifetime_wants { Some(lifetime_want_vec.len() as u32) } else { None })]
    lifetime_want_count: Option<u32>,
    #[br(if(version != SwafVersion::One))]
    #[br(count = if let Some(count) = lifetime_want_count { count } else { 0 } )]
    pub lifetime_wants: Option<Vec<WantRecord>>,
    #[br(if(version != SwafVersion::One))]
    pub max_wants: Option<u32>,
    #[br(temp)]
    #[bw(calc = wants.len() as u32)]
    want_count: u32,
    #[br(count = want_count as usize)]
    pub wants: Vec<WantRecord>,
    #[br(if(version != SwafVersion::One))]
    pub max_fears: Option<u32>,
    #[br(temp)]
    #[bw(calc = fears.len() as u32)]
    fear_count: u32,
    #[br(count = fear_count as usize)]
    pub fears: Vec<WantRecord>,
    #[br(if(version != SwafVersion::One))]
    pub unknown_1: Option<u32>,
    pub unknown_2: u32,
    pub counter: u32,
    #[br(temp)]
    #[bw(calc = previous_wants_fears.len() as u32)]
    previous_count: u32,
    #[br(count = previous_count as usize)]
    pub previous_wants_fears: Vec<PreviousWantsFears>,
}

//TODO: There probably is a better way to do this but I couldn't figure it out.
prop_compose! {
    fn swaf_mapper()(
        version in any::<SwafVersion>(),
        lifetime_wants in prop::collection::vec(any::<WantRecord>(), 0..100),
        max_wants in any::<u32>(),
        wants in prop::collection::vec(any::<WantRecord>(), 0..10),
        max_fears in any::<u32>(),
        fears in prop::collection::vec(any::<WantRecord>(), 0..10),
        unknown_1 in any::<u32>(),
        unknown_2 in any::<u32>(),
        counter in any::<u32>(),
        previous_wants_fears in prop::collection::vec(any::<PreviousWantsFears>(), 0..100),
    ) -> Swaf {
        if version != SwafVersion::One {
            Swaf {
                version,
                lifetime_wants: Some(lifetime_wants),
                max_wants: Some(max_wants),
                wants,
                max_fears: Some(max_fears),
                fears,
                unknown_1: Some(unknown_1),
                unknown_2,
                counter,
                previous_wants_fears,
            }
        } else {
            Swaf {
                version,
                lifetime_wants: None,
                max_wants: None,
                wants,
                max_fears: None,
                fears,
                unknown_1: None,
                unknown_2,
                counter,
                previous_wants_fears,
            }
        }
    }
}

impl Arbitrary for Swaf {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        swaf_mapper().boxed()
    }

    type Strategy = BoxedStrategy<Swaf>;
}

#[binrw]
#[derive(Arbitrary, Debug, PartialEq)]
#[brw(little)]
pub enum SwafVersion {
    #[brw(magic(0x01u32))]
    One,
    #[brw(magic(0x05u32))]
    Five,
    #[brw(magic(0x06u32))]
    Six,
}

#[binrw]
#[derive(Arbitrary, Debug, PartialEq)]
#[brw(little)]
pub struct PreviousWantsFears {
    pub id: u32,
    #[br(temp)]
    #[bw(calc = records.len() as u32)]
    count: u32,
    #[br(count = count as usize)]
    pub records: Vec<WantRecord>,
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
pub struct WantRecord {
    pub version: WantRecordVersion,
    pub sim_instance_id: u16,
    pub want_id: u32,
    pub want_type: WantType,
    pub price: u16,
    pub counter: u32,
    pub aspiration: i32,
    #[br(if(version == WantRecordVersion::Nine))]
    pub influence: Option<i32>,
    pub flags: u8,
}

impl Arbitrary for WantRecord {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            any::<WantRecordVersion>(),
            any::<u16>(),
            any::<u32>(),
            any::<WantType>(),
            any::<u16>(),
            any::<u32>(),
            any::<i32>(),
            any::<i32>(),
            any::<u8>(),
        )
            .prop_map(
                |(
                    version,
                    sim_instance_id,
                    want_id,
                    want_type,
                    price,
                    counter,
                    aspiration,
                    influence,
                    flags,
                )| {
                    if version == WantRecordVersion::Nine {
                        WantRecord {
                            version,
                            sim_instance_id,
                            want_id,
                            want_type,
                            price,
                            counter,
                            aspiration,
                            influence: Some(influence),
                            flags,
                        }
                    } else {
                        WantRecord {
                            version,
                            sim_instance_id,
                            want_id,
                            want_type,
                            price,
                            counter,
                            aspiration,
                            influence: None,
                            flags,
                        }
                    }
                },
            )
            .boxed()
    }

    type Strategy = BoxedStrategy<WantRecord>;
}

fn valid_want_record(input: &WantRecord) -> bool {
    if input.version == WantRecordVersion::Nine {
        input.influence.is_some()
    } else {
        input.influence.is_none()
    }
}

#[binrw]
#[derive(Arbitrary, Copy, Clone, Debug, PartialEq)]
#[brw(little)]
pub enum WantRecordVersion {
    #[brw(magic(0x07u32))]
    Seven,
    #[brw(magic(0x08u32))]
    Eight,
    #[brw(magic(0x09u32))]
    Nine,
}

#[binrw]
#[derive(Arbitrary, Copy, Clone, Debug, PartialEq)]
#[brw(little)]
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

mod tests {
    use super::*;
    use crate::test_helpers::test_parsing;
    use binrw::io::Cursor;
    use paste::paste;
    use test_strategy::proptest;

    #[proptest]
    fn want_type_symmetrical(x: WantType) {
        let mut cursor = Cursor::new(vec![]);
        cursor.write_le(&x).unwrap();

        cursor.set_position(0);

        let out: WantType = cursor.read_le().unwrap();
        assert_eq!(out, x)
    }

    #[test]
    fn want_type_parses() {
        let none: WantType = Cursor::new([0x00u8, 0xFFu8]).read_le().unwrap();
        assert_eq!(none, WantType::None);

        let sim: WantType = Cursor::new([0x01u8, 0x05u8, 0x00u8, 0xFFu8])
            .read_le()
            .unwrap();
        assert_eq!(sim, WantType::Sim(5));

        let object: WantType = Cursor::new([0x02u8, 0x05u8, 0x00u8, 0x00u8, 0x00u8, 0xFFu8])
            .read_le()
            .unwrap();
        assert_eq!(object, WantType::Object(5));

        let category: WantType = Cursor::new([0x03u8, 0x05u8, 0x00u8, 0x00u8, 0x00u8, 0xFFu8])
            .read_le()
            .unwrap();
        assert_eq!(category, WantType::Category(5));

        let skill: WantType = Cursor::new([0x04u8, 0x05u8, 0x00u8, 0xFFu8, 0xFFu8, 0xFFu8])
            .read_le()
            .unwrap();
        assert_eq!(skill, WantType::Skill(5));

        let career: WantType = Cursor::new([0x05u8, 0x05u8, 0x00u8, 0x00u8, 0x00u8, 0xFFu8])
            .read_le()
            .unwrap();
        assert_eq!(career, WantType::Career(5));
    }

    #[test]
    fn want_type_writes() {
        let mut writer_none = Cursor::new(vec![]);
        writer_none.write_le(&WantType::None).unwrap();

        assert_eq!(&writer_none.into_inner()[..], [0x00]);

        let mut writer_sim = Cursor::new(vec![]);
        writer_sim.write_le(&WantType::Sim(5)).unwrap();

        assert_eq!(&writer_sim.into_inner()[..], [0x01, 0x05, 0x00]);

        let mut writer_object = Cursor::new(vec![]);
        writer_object.write_le(&WantType::Object(5)).unwrap();

        assert_eq!(
            &writer_object.into_inner()[..],
            [0x02, 0x05, 0x00, 0x00, 0x00]
        );

        let mut writer_category = Cursor::new(vec![]);
        writer_category.write_le(&WantType::Category(5)).unwrap();

        assert_eq!(
            &writer_category.into_inner()[..],
            [0x03, 0x05, 0x00, 0x00, 0x00]
        );

        let mut writer_skill = Cursor::new(vec![]);
        writer_skill.write_le(&WantType::Skill(5)).unwrap();

        assert_eq!(&writer_skill.into_inner()[..], [0x04, 0x05, 0x00]);

        let mut writer_career = Cursor::new(vec![]);
        writer_career.write_le(&WantType::Career(5)).unwrap();

        assert_eq!(
            &writer_career.into_inner()[..],
            [0x05, 0x05, 0x00, 0x00, 0x00]
        );
    }

    test_parsing!(
        [
            0x05, 0x00, 0x00, 0x00, //want_id
            0x01, 0x00, 0x00, 0x00, //count --- header ends here
            0x09, 0x00, 0x00, 0x00, // version
            0x08, 0x00, // sim_instance_id
            0x06, 0x00, 0x00, 0x00, // want_id
            0x01, 0x07, 0x00, // want_type
            0x0A, 0x00, // price
            0x09, 0x00, 0x00, 0x00, // counter
            0x04, 0x00, 0x00, 0x00, // influence
            0x05, 0x00, 0x00, 0x00, // aspiration
            0x01, //flags
        ],
        PreviousWantsFears {
            id: 5,
            records: vec![WantRecord {
                version: WantRecordVersion::Nine,
                sim_instance_id: 8,
                want_id: 6,
                want_type: WantType::Sim(7),
                price: 10,
                counter: 9,
                aspiration: 4,
                influence: Some(5),
                flags: 1,
            }],
        },
        PreviousWantsFears,
        previous_wants_fears
    );

    test_parsing!(
        [
            0x06, 0x00, 0x00, 0x00, //Version
            0x01, 0x00, 0x00, 0x00, // ltw count
            0x09, 0x00, 0x00, 0x00, // version --------- first ltw
            0x08, 0x00, // sim_instance_id
            0x20, 0x00, 0x00, 0x00, // want_id
            0x01, 0x07, 0x00, // want_type
            0x0A, 0x00, // price
            0x09, 0x00, 0x00, 0x00, // counter
            0x04, 0x00, 0x00, 0x00, // aspiration
            0x05, 0x00, 0x00, 0x00, // influence
            0x01, //flags
            0x04, 0x00, 0x00, 0x00, // max wants
            0x01, 0x00, 0x00, 0x00, // want counts
            0x09, 0x00, 0x00, 0x00, // version ---------- wants
            0x30, 0x00, // sim_instance_id
            0x06, 0x00, 0x00, 0x00, // want_id
            0x04, 0x07, 0x00, // want_type
            0x0A, 0x00, // price
            0x09, 0x00, 0x00, 0x00, // counter
            0x04, 0x00, 0x00, 0x00, // aspiration
            0x05, 0x00, 0x00, 0x00, // influence
            0x01, //flags
            0x03, 0x00, 0x00, 0x00, // max fears
            0x01, 0x00, 0x00, 0x00, // fear count
            0x09, 0x00, 0x00, 0x00, // version ---------- fears
            0x08, 0x00, // sim_instance_id
            0x40, 0x00, 0x00, 0x00, // want_id
            0x02, 0x07, 0x00, 0x00, 0x00, // want_type
            0x0A, 0x00, // price
            0x09, 0x00, 0x00, 0x00, // counter
            0x04, 0x00, 0x00, 0x00, // aspiration
            0x05, 0x00, 0x00, 0x00, // influence
            0x01, //flags
            0x1A, 0x00, 0x00, 0x00, // unknown
            0x14, 0x00, 0x00, 0x00, // unknown
            0x0A, 0x00, 0x00, 0x00, // counter
            0x01, 0x00, 0x00, 0x00, // past record count
            0x05, 0x00, 0x00, 0x00, //want_id
            0x01, 0x00, 0x00, 0x00, //count --- header ends here
            0x09, 0x00, 0x00, 0x00, // version
            0x08, 0x00, // sim_instance_id
            0x06, 0x00, 0x00, 0x00, // want_id
            0x01, 0x07, 0x00, // want_type
            0x0A, 0x00, // price
            0x09, 0x00, 0x00, 0x00, // counter
            0x04, 0x00, 0x00, 0x00, // influence
            0x05, 0x00, 0x00, 0x00, // aspiration
            0x01, //flags
        ],
        Swaf {
            version: SwafVersion::Six,
            lifetime_wants: Some(vec![WantRecord {
                version: WantRecordVersion::Nine,
                sim_instance_id: 8,
                want_id: 32,
                want_type: WantType::Sim(7),
                price: 10,
                counter: 9,
                aspiration: 4,
                influence: Some(5),
                flags: 1,
            }]),
            max_wants: Some(4),
            wants: vec![WantRecord {
                version: WantRecordVersion::Nine,
                sim_instance_id: 48,
                want_id: 6,
                want_type: WantType::Skill(7),
                price: 10,
                counter: 9,
                aspiration: 4,
                influence: Some(5),
                flags: 1,
            },],
            max_fears: Some(3),
            fears: vec![WantRecord {
                version: WantRecordVersion::Nine,
                sim_instance_id: 8,
                want_id: 64,
                want_type: WantType::Object(7),
                price: 10,
                counter: 9,
                aspiration: 4,
                influence: Some(5),
                flags: 1,
            },],
            unknown_1: Some(26),
            unknown_2: 20,
            counter: 10,
            previous_wants_fears: vec![PreviousWantsFears {
                id: 5,
                records: vec![WantRecord {
                    version: WantRecordVersion::Nine,
                    sim_instance_id: 8,
                    want_id: 6,
                    want_type: WantType::Sim(7),
                    price: 10,
                    counter: 9,
                    aspiration: 4,
                    influence: Some(5),
                    flags: 1,
                }],
            }]
        },
        Swaf,
        swaf
    );
}
