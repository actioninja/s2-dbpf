////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;

#[cfg(test)]
use proptest::collection::vec;
#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest::sample::size_range;
use std::io::{Read, Seek, Write};
#[cfg(test)]
use test_strategy::Arbitrary;

#[binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
#[brw(little)]
pub struct Bhav {
    #[br(map(|x: NullString| x.into_string()))]
    #[bw(map(|x: &String| NullString::from_string(x.clone()) ))]
    #[brw(pad_size_to = 64)]
    #[cfg_attr(test, strategy("[\\x01-\\xFF]{1,64}"))] //non-null ascii characters only
    pub file_name: String,
    pub signature: BhavSignature,
    #[br(temp)]
    #[bw(calc = instructions.len() as u16)]
    num_instructions: u16,
    pub tree_type: u8,
    pub num_parameters: u8,
    pub num_locals: u8,
    pub flag: u8,
    pub tree_version: i32,
    #[br(args { count: num_instructions as usize, inner: (signature,) } )]
    #[bw(args_raw = (*signature,))]
    #[cfg_attr(test, strategy(vec(any_with::<BhavInstruction>((#signature,)), (0..100))))]
    //non-null ascii characters only
    pub instructions: Vec<BhavInstruction>,
}

#[binrw]
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
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

impl Default for BhavSignature {
    fn default() -> Self {
        BhavSignature::Zero
    }
}

#[derive(Debug, PartialEq)]
pub struct BhavInstruction {
    pub opcode: u16,
    pub goto_true: BhavGoTo,
    pub goto_false: BhavGoTo,
    pub node_version: Option<bool>,
    pub operands: Vec<u8>,
    pub cache_flags: Option<u8>,
}

impl BinRead for BhavInstruction {
    type Args = (BhavSignature,);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let opcode = u16::read_options(reader, options, ())?;
        let signature = args.0;
        let byte_gotos = signature < BhavSignature::Seven;
        let goto_true = BhavGoTo::read_options(reader, options, (byte_gotos,))?;
        let goto_false = BhavGoTo::read_options(reader, options, (byte_gotos,))?;
        let node_version = if signature < BhavSignature::Five {
            None
        } else {
            let bool = u8::read_options(reader, options, ())?;
            Some(bool != 0)
        };
        let operands = if signature >= BhavSignature::Three {
            <Vec<u8>>::read_options(
                reader,
                options,
                VecArgs {
                    count: 16,
                    inner: (),
                },
            )?
        } else {
            <Vec<u8>>::read_options(
                reader,
                options,
                VecArgs {
                    count: 8,
                    inner: (),
                },
            )?
        };
        let cache_flags = if signature == BhavSignature::Nine {
            Some(u8::read_options(reader, options, ())?)
        } else {
            None
        };

        Ok(BhavInstruction {
            opcode,
            goto_true,
            goto_false,
            node_version,
            operands,
            cache_flags,
        })
    }
}

impl BinWrite for BhavInstruction {
    type Args = (BhavSignature,);

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        args: Self::Args,
    ) -> BinResult<()> {
        let signature = args.0;
        let byte_gotos = signature < BhavSignature::Seven;
        u16::write_options(&self.opcode, writer, options, ())?;
        BhavGoTo::write_options(&self.goto_true, writer, options, (byte_gotos,))?;
        BhavGoTo::write_options(&self.goto_false, writer, options, (byte_gotos,))?;
        if let Some(node_version) = self.node_version {
            u8::write_options(&(node_version as u8), writer, options, ())?;
        }
        <Vec<u8>>::write_options(&self.operands, writer, options, ())?;
        <Option<u8>>::write_options(&self.cache_flags, writer, options, ())?;
        Ok(())
    }
}

#[cfg(test)]
prop_compose! {
    fn bhav_instruction_mapper(
        signature: BhavSignature
    )(
        opcode in any::<u16>(),
        goto_true in any::<BhavGoTo>(),
        goto_false in any::<BhavGoTo>(),
        node_version in any::<bool>(),
        operands_8 in any_with::<Vec<u8>>(size_range(8).lift()),
        operands_16 in any_with::<Vec<u8>>(size_range(16).lift()),
        cache_flags in any::<u8>(),
    ) -> BhavInstruction {
        BhavInstruction {
            opcode,
            goto_true,
            goto_false,
            node_version: if signature >= BhavSignature::Five {
                Some(node_version)
            } else {
                None
            },
            operands: if signature >= BhavSignature::Three {
                operands_16
            } else {
                operands_8
            },
            cache_flags: if signature >= BhavSignature::Nine {
                Some(cache_flags)
            } else {
                None
            },
        }
    }
}

#[cfg(test)]
impl Arbitrary for BhavInstruction {
    type Parameters = (BhavSignature,);

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        bhav_instruction_mapper(args.0).boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BhavGoTo {
    Error,
    True,
    False,
    OpNum(u16),
}

#[cfg(test)]
impl Arbitrary for BhavGoTo {
    type Parameters = ();

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(BhavGoTo::Error),
            Just(BhavGoTo::True),
            Just(BhavGoTo::False),
            (0..253u16).prop_map(BhavGoTo::OpNum),
        ]
        .boxed()
    }

    type Strategy = BoxedStrategy<Self>;
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
            u8::write_options(&number, writer, options, ())?;
            Ok(())
        } else {
            let number = match self {
                BhavGoTo::Error => BhavGoTo::ERROR_WORD,
                BhavGoTo::True => BhavGoTo::TRUE_WORD,
                BhavGoTo::False => BhavGoTo::FALSE_WORD,
                BhavGoTo::OpNum(op) => *op,
            };
            u16::write_options(&number, writer, options, ())?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{test_parsing, test_parsing_bhav_ins};
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

    test_parsing_bhav_ins!(
        [
            0x10,
            0x10,                 // opcode
            BhavGoTo::ERROR_BYTE, // goto true
            BhavGoTo::ERROR_BYTE, // goto false
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
        ],
        BhavInstruction {
            opcode: 0x1010,
            goto_true: BhavGoTo::Error,
            goto_false: BhavGoTo::Error,
            node_version: None,
            operands: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            cache_flags: None
        },
        BhavInstruction,
        bhav_instruction_1_range,
        (BhavSignature::Zero,)
    );

    test_parsing_bhav_ins!(
        [
            0x10,
            0x10,                 // opcode
            BhavGoTo::ERROR_BYTE, // goto true
            BhavGoTo::ERROR_BYTE, // goto false
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
            0x09,
            0x0A,
            0x0B,
            0x0C,
            0x0D,
            0x0E,
            0x0F,
            0x10,
        ],
        BhavInstruction {
            opcode: 0x1010,
            goto_true: BhavGoTo::Error,
            goto_false: BhavGoTo::Error,
            node_version: None,
            operands: vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
            cache_flags: None
        },
        BhavInstruction,
        bhav_instruction_2_range,
        (BhavSignature::Three,)
    );

    test_parsing_bhav_ins!(
        [
            0x10,
            0x10,                 // opcode
            BhavGoTo::ERROR_BYTE, // goto true
            BhavGoTo::ERROR_BYTE, // goto false
            0x00,                 // node version
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
            0x09,
            0x0A,
            0x0B,
            0x0C,
            0x0D,
            0x0E,
            0x0F,
            0x10,
        ],
        BhavInstruction {
            opcode: 0x1010,
            goto_true: BhavGoTo::Error,
            goto_false: BhavGoTo::Error,
            node_version: Some(false),
            operands: vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
            cache_flags: None
        },
        BhavInstruction,
        bhav_instruction_3_range,
        (BhavSignature::Five,)
    );

    test_parsing_bhav_ins!(
        [
            0x10, 0x10, // opcode
            0xFE, 0xFF, // goto true
            0xFE, 0xFF, // goto false
            0x00, // node version
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ],
        BhavInstruction {
            opcode: 0x1010,
            goto_true: BhavGoTo::False,
            goto_false: BhavGoTo::False,
            node_version: Some(false),
            operands: vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
            cache_flags: None
        },
        BhavInstruction,
        bhav_instruction_4_range,
        (BhavSignature::Seven,)
    );

    test_parsing_bhav_ins!(
        [
            0x10, 0x10, // opcode
            0xFE, 0xFF, // goto true
            0xFE, 0xFF, // goto false
            0x00, // node version
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, // operands
            0x00, //cache flags
        ],
        BhavInstruction {
            opcode: 0x1010,
            goto_true: BhavGoTo::False,
            goto_false: BhavGoTo::False,
            node_version: Some(false),
            operands: vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
            cache_flags: Some(0x00)
        },
        BhavInstruction,
        bhav_instruction_5_range,
        (BhavSignature::Nine,)
    );

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
            0x07, 0x80, // Signature -- LE
            0x01, 0x00, // number of instructions -- LE
            0x01, // tree type
            0x00, // num parameters
            0x00, // num locals
            0x00, // flags
            0x08, 0x00, 0x00, 0x00, // tree version -- LE i32
            0x10, 0x10, // opcode -- begin instruction
            0xFE, 0xFF, // goto true
            0xFE, 0xFF, // goto false
            0x00, // node version
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ],
        Bhav {
            file_name: "TestFile".to_string(),
            signature: BhavSignature::Seven,
            tree_type: 1,
            num_parameters: 0,
            num_locals: 0,
            flag: 0,
            tree_version: 8,
            instructions: vec![BhavInstruction {
                opcode: 0x1010,
                goto_true: BhavGoTo::False,
                goto_false: BhavGoTo::False,
                node_version: Some(false),
                operands: vec![
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                    0x0E, 0x0F, 0x10,
                ],
                cache_flags: None
            },]
        },
        Bhav,
        bhav
    );
}
