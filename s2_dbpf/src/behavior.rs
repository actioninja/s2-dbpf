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

struct Behavior {
    constants: SlotMap<ConstantKey, Constant>,
    variables: SlotMap<VariableKey, Variable>,
    instructions: SlotMap<InstructionKey, Instruction>,
    head: InstructionKey,
}

enum InstructionOutcome {
    GoTo(InstructionKey),
    ReturnTrue,
    ReturnFalse,
    Error,
}

struct Instruction {
    opcode: u16,
    params: String,
}

pub struct Constant {
    pub name: String,
    pub value: f32,
}

pub struct Variable {
    pub name: String,
    pub value: f32,
}
