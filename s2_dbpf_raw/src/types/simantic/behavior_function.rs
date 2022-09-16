////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use std::io::{Read, Seek, Write};

use crate::constants::data_kinds::{DbpfEntry, DbpfId};
use crate::types::util::parser_args::ParserArgs;
use binrw::{binrw, BinRead, BinResult, BinWrite, NullString, ReadOptions, VecArgs, WriteOptions};
#[cfg(test)]
use proptest::collection::vec;
#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest::sample::size_range;
#[cfg(test)]
use test_strategy::Arbitrary;

pub type BHAV = BehaviorFunction;

#[binrw]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
#[brw(little)]
#[br(import_raw(_args: ParserArgs))]
pub struct BehaviorFunction {
    #[br(try_map(NullString::try_into))]
    #[bw(map(| x: &String | NullString::from(x.clone())))]
    #[brw(pad_size_to = 64)]
    #[cfg_attr(test, strategy("[\\x01-\\xFF]{1,64}"))] //non-null ascii characters only
    pub file_name: String,
    pub signature: Signature,
    #[br(temp)]
    #[bw(calc = instructions.len() as u16)]
    num_instructions: u16,
    pub tree_type: u8,
    pub num_parameters: u8,
    pub num_locals: u8,
    pub flag: u8,
    pub tree_version: i32,
    #[br(args { count: num_instructions as usize, inner: (signature,) })]
    #[bw(args_raw = (* signature,))]
    #[cfg_attr(test, strategy(vec(any_with::< Instruction > ((# signature,)), (0..100))))]
    pub instructions: Vec<Instruction>,
}

impl DbpfEntry for BehaviorFunction {
    fn id(&self) -> DbpfId {
        DbpfId::BehaviorFunction
    }

    fn name(&self) -> Option<String> {
        Some(self.file_name.clone())
    }
}

#[binrw]
#[derive(Debug, PartialOrd, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
#[brw(little)]
pub enum Signature {
    #[brw(magic(0x8000_u16))]
    Zero,
    #[brw(magic(0x8001_u16))]
    One,
    #[brw(magic(0x8002_u16))]
    Two,
    #[brw(magic(0x8003_u16))]
    Three,
    #[brw(magic(0x8004_u16))]
    Four,
    #[brw(magic(0x8005_u16))]
    Five,
    #[brw(magic(0x8006_u16))]
    Six,
    #[brw(magic(0x8007_u16))]
    Seven,
    #[brw(magic(0x8008_u16))]
    Eight,
    #[brw(magic(0x8009_u16))]
    Nine,
}

impl Default for Signature {
    fn default() -> Self {
        Signature::Zero
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: u16,
    pub goto_true: GoTo,
    pub goto_false: GoTo,
    pub node_version: Option<bool>,
    pub operands: Vec<u8>,
    pub cache_flags: Option<u8>,
}

impl BinRead for Instruction {
    type Args = (Signature,);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let opcode = u16::read_options(reader, options, ())?;
        let signature = args.0;
        let byte_gotos = signature < Signature::Seven;
        let goto_true = GoTo::read_options(reader, options, (byte_gotos,))?;
        let goto_false = GoTo::read_options(reader, options, (byte_gotos,))?;
        let node_version = if signature < Signature::Five {
            None
        } else {
            let bool = u8::read_options(reader, options, ())?;
            Some(bool != 0)
        };
        let operands = if signature >= Signature::Three {
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
        let cache_flags = if signature == Signature::Nine {
            Some(u8::read_options(reader, options, ())?)
        } else {
            None
        };

        Ok(Instruction {
            opcode,
            goto_true,
            goto_false,
            node_version,
            operands,
            cache_flags,
        })
    }
}

impl BinWrite for Instruction {
    type Args = (Signature,);

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        args: Self::Args,
    ) -> BinResult<()> {
        let signature = args.0;
        let byte_gotos = signature < Signature::Seven;
        u16::write_options(&self.opcode, writer, options, ())?;
        GoTo::write_options(&self.goto_true, writer, options, (byte_gotos,))?;
        GoTo::write_options(&self.goto_false, writer, options, (byte_gotos,))?;
        if let Some(node_version) = self.node_version {
            u8::write_options(&u8::from(node_version), writer, options, ())?;
        }
        <Vec<u8>>::write_options(&self.operands, writer, options, ())?;
        <Option<u8>>::write_options(&self.cache_flags, writer, options, ())?;
        Ok(())
    }
}

#[cfg(test)]
prop_compose! {
    fn bhav_instruction_mapper(
        signature: Signature
    )(
        opcode in any::<u16>(),
        goto_true in any::<GoTo>(),
        goto_false in any::<GoTo>(),
        node_version in any::<bool>(),
        operands_8 in any_with::<Vec<u8>>(size_range(8).lift()),
        operands_16 in any_with::<Vec<u8>>(size_range(16).lift()),
        cache_flags in any::<u8>(),
    ) -> Instruction {
        Instruction {
            opcode,
            goto_true,
            goto_false,
            node_version: if signature >= Signature::Five {
                Some(node_version)
            } else {
                None
            },
            operands: if signature >= Signature::Three {
                operands_16
            } else {
                operands_8
            },
            cache_flags: if signature >= Signature::Nine {
                Some(cache_flags)
            } else {
                None
            },
        }
    }
}

#[cfg(test)]
impl Arbitrary for Instruction {
    type Parameters = (Signature,);

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        bhav_instruction_mapper(args.0).boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GoTo {
    Error,
    True,
    False,
    OpNum(u16),
}

#[cfg(test)]
impl Arbitrary for GoTo {
    type Parameters = ();

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(GoTo::Error),
            Just(GoTo::True),
            Just(GoTo::False),
            (0..253_u16).prop_map(GoTo::OpNum),
        ]
        .boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}

impl GoTo {
    const ERROR_BYTE: u8 = 0xFD;
    const TRUE_BYTE: u8 = 0xFE;
    const FALSE_BYTE: u8 = 0xFF;
    const ERROR_WORD: u16 = 0xFFFC;
    const TRUE_WORD: u16 = 0xFFFD;
    const FALSE_WORD: u16 = 0xFFFE;
}

impl BinRead for GoTo {
    type Args = (bool,);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let is_byte = args.0;
        if is_byte {
            let byte = u8::read_options(reader, options, ())?;
            match byte {
                GoTo::ERROR_BYTE => Ok(GoTo::Error),
                GoTo::TRUE_BYTE => Ok(GoTo::True),
                GoTo::FALSE_BYTE => Ok(GoTo::False),
                _ => Ok(GoTo::OpNum(u16::from(byte))),
            }
        } else {
            let short = u16::read_options(reader, options, ())?;
            match short {
                GoTo::ERROR_WORD => Ok(GoTo::Error),
                GoTo::TRUE_WORD => Ok(GoTo::True),
                GoTo::FALSE_WORD => Ok(GoTo::False),
                _ => Ok(GoTo::OpNum(short)),
            }
        }
    }
}

impl BinWrite for GoTo {
    type Args = (bool,);

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        args: Self::Args,
    ) -> BinResult<()> {
        if args.0 {
            let number = match self {
                GoTo::Error => GoTo::ERROR_BYTE,
                GoTo::True => GoTo::TRUE_BYTE,
                GoTo::False => GoTo::FALSE_BYTE,
                GoTo::OpNum(num) => *num as u8,
            };
            u8::write_options(&number, writer, options, ())?;
            Ok(())
        } else {
            let number = match self {
                GoTo::Error => GoTo::ERROR_WORD,
                GoTo::True => GoTo::TRUE_WORD,
                GoTo::False => GoTo::FALSE_WORD,
                GoTo::OpNum(op) => *op,
            };
            u16::write_options(&number, writer, options, ())?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use binrw::io::Cursor;
    use binrw::{BinReaderExt, BinWriterExt};
    use test_strategy::proptest;

    use crate::test_helpers::{test_parsing, test_parsing_bhav_ins};

    use super::*;

    #[test]
    fn goto_parses() {
        let mut cursor = Cursor::new([0xFF_u8, 0x00_u8]);
        let false_byte: GoTo = cursor.read_le_args((true,)).unwrap();
        assert_eq!(false_byte, GoTo::False);
        assert_eq!(cursor.position(), 1);

        let mut cursor = Cursor::new([0xFE_u8, 0x00_u8]);
        let true_byte: GoTo = cursor.read_le_args((true,)).unwrap();
        assert_eq!(true_byte, GoTo::True);
        assert_eq!(cursor.position(), 1);

        let mut cursor = Cursor::new([0xFD_u8, 0x00_u8]);
        let error_byte: GoTo = cursor.read_le_args((true,)).unwrap();
        assert_eq!(error_byte, GoTo::Error);
        assert_eq!(cursor.position(), 1);

        let mut cursor = Cursor::new([0x10_u8, 0x00_u8]);
        let some_byte: GoTo = cursor.read_le_args((true,)).unwrap();
        assert_eq!(some_byte, GoTo::OpNum(0x10));
        assert_eq!(cursor.position(), 1);

        let mut cursor = Cursor::new([0xFE_u8, 0xFF_u8]);
        let false_short: GoTo = cursor.read_le_args((false,)).unwrap();
        assert_eq!(false_short, GoTo::False);
        assert_eq!(cursor.position(), 2);

        let mut cursor = Cursor::new([0xFD_u8, 0xFF_u8]);
        let true_short: GoTo = cursor.read_le_args((false,)).unwrap();
        assert_eq!(true_short, GoTo::True);
        assert_eq!(cursor.position(), 2);

        let mut cursor = Cursor::new([0xFC_u8, 0xFF_u8]);
        let true_short: GoTo = cursor.read_le_args((false,)).unwrap();
        assert_eq!(true_short, GoTo::Error);
        assert_eq!(cursor.position(), 2);

        let mut cursor = Cursor::new([0x10_u8, 0x10_u8]);
        let some_short: GoTo = cursor.read_le_args((false,)).unwrap();
        assert_eq!(some_short, GoTo::OpNum(0x1010));
        assert_eq!(cursor.position(), 2);
    }

    #[test]
    fn goto_writes() {
        let mut writer_false_byte = Cursor::new(vec![]);
        writer_false_byte
            .write_le_args(&GoTo::False, (true,))
            .unwrap();

        assert_eq!(&writer_false_byte.into_inner()[..], [0xFF_u8]);

        let mut writer_true_byte = Cursor::new(vec![]);
        writer_true_byte
            .write_le_args(&GoTo::True, (true,))
            .unwrap();

        assert_eq!(&writer_true_byte.into_inner()[..], [0xFE_u8]);

        let mut writer_error_byte = Cursor::new(vec![]);
        writer_error_byte
            .write_le_args(&GoTo::Error, (true,))
            .unwrap();

        assert_eq!(&writer_error_byte.into_inner()[..], [0xFD_u8]);

        let mut writer_opnum_byte = Cursor::new(vec![]);
        writer_opnum_byte
            .write_le_args(&GoTo::OpNum(0x10), (true,))
            .unwrap();

        assert_eq!(&writer_opnum_byte.into_inner()[..], [0x10_u8]);

        // shorts

        let mut writer_false_short = Cursor::new(vec![]);
        writer_false_short
            .write_le_args(&GoTo::False, (false,))
            .unwrap();

        assert_eq!(&writer_false_short.into_inner()[..], [0xFE_u8, 0xFF_u8]);

        let mut writer_true_short = Cursor::new(vec![]);
        writer_true_short
            .write_le_args(&GoTo::True, (false,))
            .unwrap();

        assert_eq!(&writer_true_short.into_inner()[..], [0xFD_u8, 0xFF_u8]);

        let mut writer_error_short = Cursor::new(vec![]);
        writer_error_short
            .write_le_args(&GoTo::Error, (false,))
            .unwrap();

        assert_eq!(&writer_error_short.into_inner()[..], [0xFC_u8, 0xFF_u8]);

        let mut writer_opnum_short = Cursor::new(vec![]);
        writer_opnum_short
            .write_le_args(&GoTo::OpNum(0x1010), (false,))
            .unwrap();

        assert_eq!(&writer_opnum_short.into_inner()[..], [0x10_u8, 0x10_u8]);
    }

    #[proptest]
    fn goto_symmetrical(x: GoTo) {
        let mut writer = Cursor::new(vec![]);
        writer.write_le_args(&x, (false,)).unwrap();

        writer.set_position(0);

        let out: GoTo = writer.read_le_args((false,)).unwrap();

        prop_assert_eq!(out, x);
    }

    test_parsing_bhav_ins!(
        [
            0x10,
            0x10,             // opcode
            GoTo::ERROR_BYTE, // goto true
            GoTo::ERROR_BYTE, // goto false
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
        ],
        Instruction {
            opcode: 0x1010,
            goto_true: GoTo::Error,
            goto_false: GoTo::Error,
            node_version: None,
            operands: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            cache_flags: None
        },
        Instruction,
        bhav_instruction_1_range,
        (Signature::Zero,)
    );

    test_parsing_bhav_ins!(
        [
            0x10,
            0x10,             // opcode
            GoTo::ERROR_BYTE, // goto true
            GoTo::ERROR_BYTE, // goto false
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
        Instruction {
            opcode: 0x1010,
            goto_true: GoTo::Error,
            goto_false: GoTo::Error,
            node_version: None,
            operands: vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
            cache_flags: None
        },
        Instruction,
        bhav_instruction_2_range,
        (Signature::Three,)
    );

    test_parsing_bhav_ins!(
        [
            0x10,
            0x10,             // opcode
            GoTo::ERROR_BYTE, // goto true
            GoTo::ERROR_BYTE, // goto false
            0x00,             // node version
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
        Instruction {
            opcode: 0x1010,
            goto_true: GoTo::Error,
            goto_false: GoTo::Error,
            node_version: Some(false),
            operands: vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
            cache_flags: None
        },
        Instruction,
        bhav_instruction_3_range,
        (Signature::Five,)
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
        Instruction {
            opcode: 0x1010,
            goto_true: GoTo::False,
            goto_false: GoTo::False,
            node_version: Some(false),
            operands: vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
            cache_flags: None
        },
        Instruction,
        bhav_instruction_4_range,
        (Signature::Seven,)
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
        Instruction {
            opcode: 0x1010,
            goto_true: GoTo::False,
            goto_false: GoTo::False,
            node_version: Some(false),
            operands: vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
            cache_flags: Some(0x00)
        },
        Instruction,
        bhav_instruction_5_range,
        (Signature::Nine,)
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
        BehaviorFunction {
            file_name: "TestFile".to_string(),
            signature: Signature::Seven,
            tree_type: 1,
            num_parameters: 0,
            num_locals: 0,
            flag: 0,
            tree_version: 8,
            instructions: vec![Instruction {
                opcode: 0x1010,
                goto_true: GoTo::False,
                goto_false: GoTo::False,
                node_version: Some(false),
                operands: vec![
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                    0x0E, 0x0F, 0x10,
                ],
                cache_flags: None
            },]
        },
        BehaviorFunction,
        bhav
    );
}
