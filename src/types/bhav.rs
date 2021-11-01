////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;
#[cfg(test)]
use proptest::prelude::*;
use std::io::{Read, Seek, Write};
#[cfg(test)]
use test_strategy::Arbitrary;

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little)]
struct Bhav {
    pub file_name: NullString,
    #[brw(pad_size_to = 64)]
    pub header: BhavHeader,
    #[br(args { count: header.num_instructions as usize, inner: (header.signature,) } )]
    pub instructions: Vec<BhavInstruction>,
}

#[binrw]
#[derive(Debug, PartialOrd, PartialEq)]
#[brw(little)]
struct BhavHeader {
    signature: BhavSignature,
    num_instructions: u16,
    tree_type: u8,
    num_parameters: u8,
    num_locals: u8,
    flag: u8,
    tree_version: i32,
}

#[binrw]
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
#[brw(little)]
pub enum BhavSignature {
    #[brw(magic(0x8000u16))]
    Zero,
    #[brw(magic(0x8001u16))]
    One,
    #[brw(magic(0x8002u16))]
    Two,
    #[brw(magic(0x8003u16))]
    Three,
    #[brw(magic(0x8004u16))]
    Four,
    #[brw(magic(0x8005u16))]
    Five,
    #[brw(magic(0x8006u16))]
    Six,
    #[brw(magic(0x8007u16))]
    Seven,
    #[brw(magic(0x8008u16))]
    Eight,
    #[brw(magic(0x8009u16))]
    Nine,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BhavInstruction {
    opcode: u16,
    goto_true: BhavGoTo,
    goto_false: BhavGoTo,
    node_version: Option<bool>,
    operands: [u8; 16],
    cache_flags: Option<u8>,
}

impl BinRead for BhavInstruction {
    type Args = (BhavSignature,);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        todo!()
    }
}

impl BinWrite for BhavInstruction {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        args: Self::Args,
    ) -> BinResult<()> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum BhavGoTo {
    Error,
    True,
    False,
    OpNum(u16),
}

impl BhavGoTo {
    const ERROR_BYTE: u8 = 0xFD;
    const TRUE_BYTE: u8 = 0xFE;
    const FALSE_BYTE: u8 = 0xFF;
    const ERROR_WORD: u16 = 0xFFFC;
    const TRUE_WORD: u16 = 0xFFFD;
    const FALSE_WORD: u16 = 0xFFFE;
}

impl BinRead for BhavGoTo {
    type Args = (bool,);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        if args.0 {
            let byte = u8::read_options(reader, options, ())?;
            match byte {
                BhavGoTo::ERROR_BYTE => Ok(BhavGoTo::Error),
                BhavGoTo::TRUE_BYTE => Ok(BhavGoTo::True),
                BhavGoTo::FALSE_BYTE => Ok(BhavGoTo::False),
                _ => Ok(BhavGoTo::OpNum(byte as u16)),
            }
        } else {
            let short = u16::read_options(reader, options, ())?;
            match short {
                BhavGoTo::ERROR_WORD => Ok(BhavGoTo::Error),
                BhavGoTo::TRUE_WORD => Ok(BhavGoTo::True),
                BhavGoTo::FALSE_WORD => Ok(BhavGoTo::False),
                _ => Ok(BhavGoTo::OpNum(short)),
            }
        }
    }
}

impl BinWrite for BhavGoTo {
    type Args = (bool,);

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        args: Self::Args,
    ) -> BinResult<()> {
        if args.0 {
            let number = match self {
                BhavGoTo::Error => BhavGoTo::ERROR_BYTE,
                BhavGoTo::True => BhavGoTo::TRUE_BYTE,
                BhavGoTo::False => BhavGoTo::FALSE_BYTE,
                BhavGoTo::OpNum(num) => *num as u8,
            };
            u8::write_options(&number, writer, &options, ());
            Ok(())
        } else {
            let number = match self {
                BhavGoTo::Error => BhavGoTo::ERROR_WORD,
                BhavGoTo::True => BhavGoTo::TRUE_WORD,
                BhavGoTo::False => BhavGoTo::FALSE_WORD,
                BhavGoTo::OpNum(op) => *op,
            };
            u16::write_options(&number, writer, &options, ());
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::test_parsing;
    use binrw::io::Cursor;
    use paste::paste;
    use test_strategy::proptest;

    #[test]
    fn goto_parses() {
        let mut cursor = Cursor::new([0xFFu8, 0x00u8]);
        let false_byte: BhavGoTo = cursor.read_le_args((true,)).unwrap();
        assert_eq!(false_byte, BhavGoTo::False);
        assert_eq!(cursor.position(), 1);

        let mut cursor = Cursor::new([0xFEu8, 0x00u8]);
        let true_byte: BhavGoTo = cursor.read_le_args((true,)).unwrap();
        assert_eq!(true_byte, BhavGoTo::True);
        assert_eq!(cursor.position(), 1);

        let mut cursor = Cursor::new([0xFDu8, 0x00u8]);
        let error_byte: BhavGoTo = cursor.read_le_args((true,)).unwrap();
        assert_eq!(error_byte, BhavGoTo::Error);
        assert_eq!(cursor.position(), 1);

        let mut cursor = Cursor::new([0x10u8, 0x00u8]);
        let some_byte: BhavGoTo = cursor.read_le_args((true,)).unwrap();
        assert_eq!(some_byte, BhavGoTo::OpNum(0x10));
        assert_eq!(cursor.position(), 1);

        let mut cursor = Cursor::new([0xFEu8, 0xFFu8]);
        let false_short: BhavGoTo = cursor.read_le_args((false,)).unwrap();
        assert_eq!(false_short, BhavGoTo::False);
        assert_eq!(cursor.position(), 2);

        let mut cursor = Cursor::new([0xFDu8, 0xFFu8]);
        let true_short: BhavGoTo = cursor.read_le_args((false,)).unwrap();
        assert_eq!(true_short, BhavGoTo::True);
        assert_eq!(cursor.position(), 2);

        let mut cursor = Cursor::new([0xFCu8, 0xFFu8]);
        let true_short: BhavGoTo = cursor.read_le_args((false,)).unwrap();
        assert_eq!(true_short, BhavGoTo::Error);
        assert_eq!(cursor.position(), 2);

        let mut cursor = Cursor::new([0x10u8, 0x10u8]);
        let some_short: BhavGoTo = cursor.read_le_args((false,)).unwrap();
        assert_eq!(some_short, BhavGoTo::OpNum(0x1010));
        assert_eq!(cursor.position(), 2);
    }

    #[test]
    fn goto_writes() {
        let mut writer_false_byte = Cursor::new(vec![]);
        writer_false_byte
            .write_le_args(&BhavGoTo::False, (true,))
            .unwrap();

        assert_eq!(&writer_false_byte.into_inner()[..], [0xFFu8]);

        let mut writer_true_byte = Cursor::new(vec![]);
        writer_true_byte
            .write_le_args(&BhavGoTo::True, (true,))
            .unwrap();

        assert_eq!(&writer_true_byte.into_inner()[..], [0xFEu8]);

        let mut writer_error_byte = Cursor::new(vec![]);
        writer_error_byte
            .write_le_args(&BhavGoTo::Error, (true,))
            .unwrap();

        assert_eq!(&writer_error_byte.into_inner()[..], [0xFDu8]);

        let mut writer_opnum_byte = Cursor::new(vec![]);
        writer_opnum_byte
            .write_le_args(&BhavGoTo::OpNum(0x10), (true,))
            .unwrap();

        assert_eq!(&writer_opnum_byte.into_inner()[..], [0x10u8]);

        // shorts

        let mut writer_false_short = Cursor::new(vec![]);
        writer_false_short
            .write_le_args(&BhavGoTo::False, (false,))
            .unwrap();

        assert_eq!(&writer_false_short.into_inner()[..], [0xFEu8, 0xFFu8]);

        let mut writer_true_short = Cursor::new(vec![]);
        writer_true_short
            .write_le_args(&BhavGoTo::True, (false,))
            .unwrap();

        assert_eq!(&writer_true_short.into_inner()[..], [0xFDu8, 0xFFu8]);

        let mut writer_error_short = Cursor::new(vec![]);
        writer_error_short
            .write_le_args(&BhavGoTo::Error, (false,))
            .unwrap();

        assert_eq!(&writer_error_short.into_inner()[..], [0xFCu8, 0xFFu8]);

        let mut writer_opnum_short = Cursor::new(vec![]);
        writer_opnum_short
            .write_le_args(&BhavGoTo::OpNum(0x1010), (false,))
            .unwrap();

        assert_eq!(&writer_opnum_short.into_inner()[..], [0x10u8, 0x10u8]);
    }

    #[proptest]
    fn goto_symmetrical(x: BhavGoTo) {
        let mut writer = Cursor::new(vec![]);
        writer.write_le_args(&x, (false,)).unwrap();

        writer.set_position(0);

        let out: BhavGoTo = writer.read_le_args((false,)).unwrap();

        prop_assert_eq!(out, x);
    }

    #[test]
    fn instruction_parses() {}

    #[test]
    fn instruction_writes() {}

    #[test]
    fn instruction_symmetrical() {}

    /*
    test_parsing!(
        [
            0x07, 0x80, // signature
            0x03, 0x00, // num instruction
            0x04, // tree type
            0x05, // num parameters
            0x06, // num locals
        ],
        BhavHeader {},
        BhavHeader,
        bhav_header
    );
     */
}
