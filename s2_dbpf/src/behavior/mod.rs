////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

pub mod operation;
mod spec;

use crate::behavior::operation::Operation;
use slotmap::{new_key_type, SlotMap};

new_key_type! {
    pub struct ConstantKey;
    pub struct VariableKey;
    pub struct InstructionKey;
    pub struct ParameterKey;
}

#[derive(Clone, Default)]
pub struct Behavior {
    pub constants: SlotMap<ConstantKey, Constant>,
    pub variables: SlotMap<VariableKey, Variable>,
    pub parameters: SlotMap<ParameterKey, Variable>,
    pub instructions: SlotMap<InstructionKey, Instruction>,
    pub head: Option<InstructionKey>,
}

impl Behavior {
    pub fn new() -> Self {
        Behavior::default()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum InstructionOutcome {
    GoTo(InstructionKey),
    ReturnTrue,
    ReturnFalse,
    #[default]
    Error,
}

#[derive(Clone)]
pub struct Instruction {
    operation: Operation,
    true_outcome: InstructionOutcome,
    false_outcome: InstructionOutcome,
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
