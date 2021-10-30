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
    pub max_fears: u32,
    fear_count: u32,
    #[br(count = fear_count as usize)]
    pub fears: Vec<WantRecord>,
    #[br(if(version >= 5))]
    pub unknown_1: Option<u32>,
    pub unknown_2: u32,
    pub counter: u32,
    previous_count: u32,
    #[br(count = previous_count as usize)]
    pub previous_wants_fears: Vec<PreviousWantsFears>,
}

#[binrw]
#[derive(Arbitrary, Debug, PartialEq)]
#[brw(little)]
pub struct PreviousWantsFears {
    pub id: u32,
    count: u32,
    #[br(count = count as usize)]
    pub records: Vec<WantRecord>,
}

#[binrw]
#[derive(Arbitrary, Debug, PartialEq)]
#[filter(valid_want_record)]
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
    use binrw::io::Cursor;
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

    #[test]
    fn want_record_parses() {
        let test_record = [
            0x09, 0x00, 0x00, 0x00, // version
            0x08, 0x00, // sim_instance_id
            0x06, 0x00, 0x00, 0x00, // want_id
            0x01, 0x07, 0x00, // want_type
            0x0A, 0x00, // price
            0x09, 0x00, 0x00, 0x00, // counter
            0x04, 0x00, 0x00, 0x00, // influence
            0x05, 0x00, 0x00, 0x00, // aspiration
            0x01, //flags
        ];

        let want_record: WantRecord = Cursor::new(test_record).read_le().unwrap();
        let want_record_expected = WantRecord {
            version: WantRecordVersion::Nine,
            sim_instance_id: 8,
            want_id: 6,
            want_type: WantType::Sim(7),
            price: 10,
            counter: 9,
            aspiration: 4,
            influence: Some(5),
            flags: 1,
        };

        assert_eq!(want_record, want_record_expected);
    }

    #[test]
    fn want_record_writes() {
        let want_record_before = WantRecord {
            version: WantRecordVersion::Nine,
            sim_instance_id: 8,
            want_id: 6,
            want_type: WantType::Sim(7),
            price: 10,
            counter: 9,
            aspiration: 4,
            influence: Some(5),
            flags: 1,
        };
        let mut want_cursor = Cursor::new(vec![]);
        want_cursor.write_le(&want_record_before).unwrap();

        let test_data_expected = [
            0x09, 0x00, 0x00, 0x00, // version
            0x08, 0x00, // sim_instance_id
            0x06, 0x00, 0x00, 0x00, // want_id
            0x01, 0x07, 0x00, // want_type
            0x0A, 0x00, // price
            0x09, 0x00, 0x00, 0x00, // counter
            0x04, 0x00, 0x00, 0x00, // influence
            0x05, 0x00, 0x00, 0x00, // aspiration
            0x01, //flags
        ];

        assert_eq!(&want_cursor.into_inner()[..], test_data_expected)
    }

    #[proptest]
    fn want_record_symmetrical(x: WantRecord) {
        let mut cursor = Cursor::new(vec![]);
        cursor.write_le(&x).unwrap();

        let output = cursor.get_ref();

        cursor.set_position(0);

        let out: WantRecord = cursor.read_le().unwrap();
        prop_assert_eq!(out, x)
    }

    #[test]
    fn previous_wants_parses() {
        let before_parse = [
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
        ];

        let actual: PreviousWantsFears = Cursor::new(before_parse).read_le().unwrap();

        let expected = PreviousWantsFears {
            id: 5,
            count: 1,
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
        };

        assert_eq!(actual, expected)
    }

    test_parse!(
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
            count: 1,
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
        "previous_wants_fears"
    );
}
