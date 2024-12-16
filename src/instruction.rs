use crate::{machine::MachineState, word::Word};

enum InstructionLoadError {
    WrongInstruction
}

enum InstructionExecuteError {
    
}

trait Executable {
    pub fn execute(&self, state: MachineState) -> MachineState;

    pub fn code() -> u8;

    fn is_correct_instruction(word: Word) -> bool {
        return word.0 == Self::code();
    }

    pub fn from_word(word: Word) -> Result<Instruction, InstructionLoadError>;
}

type Instruction = Box<dyn Executable>;

impl MachineState {
    pub fn execute(&mut self, instruction: Instruction) {
        instruction.execute(self)
    }
}

pub struct NoOperation;
impl Executable for NoOperation {
    fn execute(&self, &mut state: MachineState) {
        state.instruction_pointer += 1;
    }

    fn code() -> u8 {
        0
    }

    fn from_word(word: Word) -> Result<Executable, InstructionLoadError> {
        if !Self::is_correct_instruction(word) {
            return Err(InstructionLoadError::WrongInstruction);
        }

        return NoOperation;
    }
}
