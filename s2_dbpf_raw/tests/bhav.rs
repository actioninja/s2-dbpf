////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use std::fs::File;
use std::io::BufReader;

use binrw::BinReaderExt;

use s2_dbpf_raw::types::simantic::behavior_function::{
    BehaviorFunction, GoTo, Instruction, Signature,
};

const PATH: &str = "tests/ex_files/bhav/42484156-00000000-7FB208FA-0000202E.bhav";

#[test]
fn real_file_bhav_parse() {
    let file_handle = File::open(PATH).unwrap();
    let mut buf_reader = BufReader::new(file_handle);
    let bhav: BehaviorFunction = buf_reader.read_le().unwrap();
    let expected = BehaviorFunction {
        file_name: "Interaction - Answer".to_string(),
        signature: Signature::Seven,
        tree_type: 0,
        num_parameters: 0,
        num_locals: 2,
        flag: 6,
        tree_version: -32745,
        instructions: vec![
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(1),
                goto_false: GoTo::OpNum(3),
                node_version: Some(false),
                operands: vec![59, 0, 1, 67, 0, 2, 4, 26, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(2),
                goto_false: GoTo::OpNum(2),
                node_version: Some(false),
                operands: vec![0, 0, 0, 0, 0, 5, 25, 10, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(5),
                goto_false: GoTo::OpNum(5),
                node_version: Some(false),
                operands: vec![1, 0, 0, 0, 0, 5, 25, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(4),
                goto_false: GoTo::OpNum(4),
                node_version: Some(false),
                operands: vec![1, 0, 0, 0, 0, 5, 25, 10, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(5),
                goto_false: GoTo::OpNum(5),
                node_version: Some(false),
                operands: vec![0, 0, 0, 0, 0, 5, 25, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 376,
                goto_true: GoTo::OpNum(6),
                goto_false: GoTo::False,
                node_version: Some(true),
                operands: vec![25, 1, 0, 10, 0, 0, 10, 0, 0, 10, 0, 0, 1, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(7),
                goto_false: GoTo::OpNum(7),
                node_version: Some(false),
                operands: vec![0, 0, 1, 0, 0, 5, 10, 25, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(9),
                goto_false: GoTo::OpNum(8),
                node_version: Some(false),
                operands: vec![59, 0, 39, 67, 0, 2, 4, 26, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 32,
                goto_true: GoTo::OpNum(9),
                goto_false: GoTo::False,
                node_version: Some(false),
                operands: vec![230, 8, 132, 206, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 376,
                goto_true: GoTo::OpNum(10),
                goto_false: GoTo::False,
                node_version: Some(true),
                operands: vec![25, 0, 0, 7, 0, 0, 7, 0, 0, 7, 0, 0, 1, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(11),
                goto_false: GoTo::OpNum(11),
                node_version: Some(false),
                operands: vec![0, 0, 0, 0, 0, 5, 10, 25, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(13),
                goto_false: GoTo::OpNum(12),
                node_version: Some(false),
                operands: vec![59, 0, 1, 67, 0, 2, 4, 26, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 32,
                goto_true: GoTo::OpNum(13),
                goto_false: GoTo::False,
                node_version: Some(false),
                operands: vec![230, 8, 132, 206, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 264,
                goto_true: GoTo::OpNum(14),
                goto_false: GoTo::OpNum(17),
                node_version: Some(false),
                operands: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 8,
                goto_true: GoTo::OpNum(15),
                goto_false: GoTo::OpNum(15),
                node_version: Some(false),
                operands: vec![0, 0, 8, 0, 100, 0, 7, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(16),
                goto_false: GoTo::OpNum(17),
                node_version: Some(false),
                operands: vec![0, 0, 128, 34, 0, 1, 8, 26, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(17),
                goto_false: GoTo::OpNum(17),
                node_version: Some(false),
                operands: vec![17, 0, 1, 0, 0, 5, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 8198,
                goto_true: GoTo::OpNum(18),
                goto_false: GoTo::OpNum(26),
                node_version: Some(true),
                operands: vec![7, 0, 0, 25, 1, 0, 7, 0, 0, 7, 0, 0, 1, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 8195,
                goto_true: GoTo::OpNum(19),
                goto_false: GoTo::OpNum(19),
                node_version: Some(true),
                operands: vec![25, 0, 0, 7, 0, 0, 7, 0, 0, 7, 0, 0, 1, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::OpNum(20),
                goto_false: GoTo::OpNum(23),
                node_version: Some(false),
                operands: vec![0, 0, 2, 35, 0, 2, 8, 26, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 8245,
                goto_true: GoTo::OpNum(21),
                goto_false: GoTo::OpNum(21),
                node_version: Some(true),
                operands: vec![25, 0, 0, 7, 0, 0, 7, 0, 0, 7, 0, 0, 1, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 8205,
                goto_true: GoTo::OpNum(22),
                goto_false: GoTo::OpNum(22),
                node_version: Some(true),
                operands: vec![25, 0, 0, 10, 0, 0, 10, 0, 0, 10, 0, 0, 1, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::True,
                goto_false: GoTo::True,
                node_version: Some(false),
                operands: vec![17, 0, 0, 0, 0, 5, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 8239,
                goto_true: GoTo::OpNum(24),
                goto_false: GoTo::OpNum(24),
                node_version: Some(true),
                operands: vec![25, 0, 0, 10, 0, 0, 10, 0, 0, 10, 0, 0, 1, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 8245,
                goto_true: GoTo::OpNum(25),
                goto_false: GoTo::OpNum(25),
                node_version: Some(true),
                operands: vec![25, 0, 0, 7, 0, 0, 7, 0, 0, 7, 0, 0, 1, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::True,
                goto_false: GoTo::True,
                node_version: Some(false),
                operands: vec![17, 0, 0, 0, 0, 5, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
            Instruction {
                opcode: 2,
                goto_true: GoTo::False,
                goto_false: GoTo::False,
                node_version: Some(false),
                operands: vec![17, 0, 0, 0, 0, 5, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0],
                cache_flags: None,
            },
        ],
    };
    assert_eq!(bhav, expected);
}
