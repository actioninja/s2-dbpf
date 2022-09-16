////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use slotmap::{new_key_type, SlotMap};

new_key_type! {
    struct ConstantKey;
    struct VariableKey;
    struct InstructionKey;
}

#[derive(Clone)]
struct Behavior {
    constants: SlotMap<ConstantKey, Constant>,
    variables: SlotMap<VariableKey, Variable>,
    instructions: SlotMap<InstructionKey, Instruction>,
    head: InstructionKey,
}

impl Behavior {
    pub fn new() {}
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
enum InstructionOutcome {
    GoTo(InstructionKey),
    ReturnTrue,
    ReturnFalse,
    #[default]
    Error,
}

#[derive(Clone)]
struct Instruction {
    opcode: u16,
    params: String,
}

#[derive(Clone)]
pub struct Constant {
    pub name: String,
    pub value: f32,
}

#[derive(Clone)]
pub struct Variable {
    pub name: String,
    pub value: f32,
}
