////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::*;

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
    signature: u16,
    num_instructions: u16,
    tree_type: u8,
    num_parameters: u8,
    num_locals: u8,
    flag: u8,
    tree_version: i32,
}

#[binrw]
#[derive(Debug, PartialOrd, PartialEq)]
#[brw(little)]
#[br(import(signature: u16))]
enum BhavInstruction {
    #[br(pre_assert(signature == 0x8000))]
    #[br(pre_assert(signature == 0x8001))]
    #[br(pre_assert(signature == 0x8002))]
    First {
        opcode: u16,
        goto_true: BhavGoToByte,
        goto_false: BhavGoToByte,
        operands: [u8; 8],
    },
    #[br(pre_assert(signature == 0x8003))]
    #[br(pre_assert(signature == 0x8004))]
    Second {
        opcode: u16,
        goto_true: BhavGoToByte,
        goto_false: BhavGoToByte,
        operands: [u8; 16],
    },
    #[br(pre_assert(signature == 0x8005))]
    #[br(pre_assert(signature == 0x8006))]
    Third {
        opcode: u16,
        goto_true: BhavGoToByte,
        goto_false: BhavGoToByte,
        #[br(map = |x: u8| x == 1)]
        #[bw(map = |x: &bool| *x as u8)]
        node_version: bool,
        operands: [u8; 16],
    },
    #[br(pre_assert(signature == 0x8007))]
    #[br(pre_assert(signature == 0x8008))]
    Fourth {
        opcode: u16,
        goto_true: BhavGoToShort,
        goto_false: BhavGoToShort,
        #[br(map = |x: u8| x == 1)]
        #[bw(map = |x: &bool| *x as u8)]
        node_version: bool,
        operands: [u8; 16],
    },
    #[br(pre_assert(signature == 0x8009))]
    Fifth {
        opcode: u16,
        goto_true: BhavGoToShort,
        goto_false: BhavGoToShort,
        #[br(map = |x: u8| x == 1)]
        #[bw(map = |x: &bool| *x as u8)]
        node_version: bool,
        operands: [u8; 16],
        cache_flags: u8,
    },
}

#[binrw]
#[brw(little)]
#[derive(Debug, PartialOrd, PartialEq)]
enum BhavGoToByte {
    #[brw(magic(0xFDu8))]
    Error,
    #[brw(magic(0xFEu8))]
    True,
    #[brw(magic(0xFFu8))]
    False,
    OpNum(u8),
}

#[binrw]
#[brw(little)]
#[derive(Debug, PartialOrd, PartialEq)]
enum BhavGoToShort {
    #[brw(magic(0xFFFCu16))]
    Error,
    #[brw(magic(0xFFFDu16))]
    True,
    #[brw(magic(0xFFFEu16))]
    False,
    OpNum(u16),
}

mod tests {}
