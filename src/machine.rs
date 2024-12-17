use crate::{flag::FlagState, register::RegisterSet, word::Word};

#[derive(Clone)]
pub struct MachineState {
    pub memory: Vec<Word>,
    pub instruction: u32,
    pub flags: FlagState,
    pub registers: RegisterSet,
    pub halted: bool,
}
