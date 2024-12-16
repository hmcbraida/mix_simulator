use crate::{flag::FlagState, register::RegisterSet, word::Word};

#[derive(Clone)]
pub struct MachineState {
    memory: Vec<Word>,
    instruction: u32,
    flags: FlagState,
    registers: RegisterSet
}
