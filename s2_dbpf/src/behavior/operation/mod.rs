////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct OpCode(u16);

pub mod primitives;

pub enum Operation {
    Primitive,
    Local(ExternalCall),
    Global(ExternalCall),
    SemiGlobal(ExternalCall),
}

pub struct ExternalCall {
    opcode: OpCode,
    args: Vec<u8>,
}

/*
pub trait Operation {
    fn from_params(params: &[u8]) -> Self;
    fn to_params(&self) -> [u8; 16] {
        [0x00; 16]
    }
    fn opcode(&self) -> u16;
}

pub
 */
