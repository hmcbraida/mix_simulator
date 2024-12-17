use core::{panic, panicking::panic};

use crate::{io::TapeUnit, machine::MachineState, word::Word};

pub struct InvalidInstruction;

enum Register {
    A,
    X,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
}

impl Register {
    fn from_n(value: u8) -> Self {
        match value {
            0 => Register::A,
            1 => Register::I1,
            2 => Register::I2,
            3 => Register::I3,
            4 => Register::I4,
            5 => Register::I5,
            6 => Register::I6,
            7 => Register::X,
            _ => panic!()
        }
    }
}

enum FlagCondition {
    Less,
    Equal,
    Greater,
    GreaterOrEqual,
    NotEqual,
    LessOrEqual
}

impl FlagCondition {
    fn from_f_field(value: u8) -> Result<FlagCondition, InvalidInstruction> {
        match value {
            4 => Ok(FlagCondition::Less),
            5 => Ok(FlagCondition::Equal),
            6 => Ok(FlagCondition::Greater),
            7 => Ok(FlagCondition::GreaterOrEqual),
            8 => Ok(FlagCondition::NotEqual),
            9 => Ok(FlagCondition::LessOrEqual),
            _ => Err(InvalidInstruction)
        }
    }
}

enum RegisterCondition {
    Negative,
    Zero,
    Positive,
    NotNegative,
    NotZero,
    NotPositive
}

impl RegisterCondition {
    fn from_f_field(value: u8) -> Result<RegisterCondition, InvalidInstruction> {
        match value {
            0 => Ok(RegisterCondition::Negative),
            1 => Ok(RegisterCondition::Zero),
            2 => Ok(RegisterCondition::Positive),
            3 => Ok(RegisterCondition::NotNegative),
            4 => Ok(RegisterCondition::NotZero),
            5 => Ok(RegisterCondition::NotPositive),
            _ => Err(InvalidInstruction)
        }
    }
}

enum InstructionDevice {
    TapeUnit(u8),
    Disk(u8),
    CardReader,
    CardPunch,
    LinePrinter,
    Typewriter,
    PaperTape,
}

impl InstructionDevice {
    fn from_f_field(value: u8) -> Result<InstructionDevice, InvalidInstruction> {
        match value {
            0..8 => Ok(InstructionDevice::TapeUnit(value)),
            8..16 => Ok(InstructionDevice::Disk(value - 8)),
            16 => Ok(InstructionDevice::CardReader),
            17 => Ok(InstructionDevice::CardPunch),
            18 => Ok(InstructionDevice::LinePrinter),
            19 => Ok(InstructionDevice::Typewriter),
            20 => Ok(InstructionDevice::PaperTape),
            _ => Err(InvalidInstruction),
        }
    }
}

pub enum InstructionOp {
    NoOperation,
    Add,
    Sub,
    Mul,
    Div,
    Num,
    Char,
    Halt,
    ShiftLeftA,
    ShiftRightA,
    ShiftLeftAX,
    ShiftRightAX,
    ShiftLeftCirc,
    ShiftRightCirc,
    Move,
    LoadReg(Register),
    LoadRegNeg(Register),
    StoreReg(Register),
    StoreJump,
    StoreZero,
    JumpIfBusy(InstructionDevice),
    ControlOp(InstructionDevice),
    Input(InstructionDevice),
    Output(InstructionDevice),
    JumpIfReady(InstructionDevice),
    Jump,
    JumpSaveJump,
    JumpIfOverflow,
    JumpIfNoOverflow,
    JumpIfFlag(FlagCondition),
    JumpIfReg(Register, RegisterCondition),
    IncReg(Register),
    DecReg(Register),
    EnterReg(Register),
    EnterRegNeg(Register),
    CompareReg(Register),
}

pub struct Instruction {
    op: InstructionOp,
    f: u32,
    i: u32,
    a: u32
}

pub struct InvalidInstructionWord;

impl InstructionOp {
    #[allow(clippy::too_many_lines)]
    pub fn from_word(word: &Word) -> Result<Self, InvalidInstruction> {
        let c_field = u8::try_from(word.c_field().to_number()).unwrap();
        let f_field = u8::try_from(word.f_field().to_number()).unwrap();

        match c_field {
            0 => Ok(InstructionOp::NoOperation),
            1 => Ok(InstructionOp::Add),
            2 => Ok(InstructionOp::Sub),
            3 => Ok(InstructionOp::Mul),
            4 => Ok(InstructionOp::Div),
            5 => match f_field {
                0 => Ok(InstructionOp::Num),
                1 => Ok(InstructionOp::Char),
                2 => Ok(InstructionOp::Halt),
                _ => Err(InvalidInstruction)
            },
            6 => match f_field {
                0 => Ok(InstructionOp::ShiftLeftA),
                1 => Ok(InstructionOp::ShiftRightA),
                2 => Ok(InstructionOp::ShiftLeftAX),
                3 => Ok(InstructionOp::ShiftRightAX),
                4 => Ok(InstructionOp::ShiftLeftCirc),
                5 => Ok(InstructionOp::ShiftRightCirc),
                _ => Err(InvalidInstruction)
            },
            7 => Ok(InstructionOp::Move),
            8 => Ok(InstructionOp::LoadReg(Register::A)),
            9 => Ok(InstructionOp::LoadReg(Register::I1)),
            10 => Ok(InstructionOp::LoadReg(Register::I2)),
            11 => Ok(InstructionOp::LoadReg(Register::I3)),
            12 => Ok(InstructionOp::LoadReg(Register::I4)),
            13 => Ok(InstructionOp::LoadReg(Register::I5)),
            14 => Ok(InstructionOp::LoadReg(Register::I6)),
            15 => Ok(InstructionOp::LoadReg(Register::X)),
            16 => Ok(InstructionOp::LoadRegNeg(Register::A)),
            17 => Ok(InstructionOp::LoadRegNeg(Register::I1)),
            18 => Ok(InstructionOp::LoadRegNeg(Register::I2)),
            19 => Ok(InstructionOp::LoadRegNeg(Register::I3)),
            20 => Ok(InstructionOp::LoadRegNeg(Register::I4)),
            21 => Ok(InstructionOp::LoadRegNeg(Register::I5)),
            22 => Ok(InstructionOp::LoadRegNeg(Register::I6)),
            23 => Ok(InstructionOp::LoadRegNeg(Register::X)),
            24 => Ok(InstructionOp::StoreReg(Register::A)),
            25 => Ok(InstructionOp::StoreReg(Register::I1)),
            26 => Ok(InstructionOp::StoreReg(Register::I2)),
            27 => Ok(InstructionOp::StoreReg(Register::I3)),
            28 => Ok(InstructionOp::StoreReg(Register::I4)),
            29 => Ok(InstructionOp::StoreReg(Register::I5)),
            30 => Ok(InstructionOp::StoreReg(Register::I6)),
            31 => Ok(InstructionOp::StoreReg(Register::X)),
            32 => Ok(InstructionOp::StoreJump),
            33 => Ok(InstructionOp::StoreZero),
            34 => {
                let device = InstructionDevice::from_f_field(f_field)?;

                Ok(InstructionOp::JumpIfBusy(device))
            },
            35 => {
                let device = InstructionDevice::from_f_field(f_field)?;
                
                Ok(InstructionOp::ControlOp(device))
            },
            36 => {
                let device = InstructionDevice::from_f_field(f_field)?;
                
                Ok(InstructionOp::Input(device))
            },
            37 => {
                let device = InstructionDevice::from_f_field(f_field)?;
                
                Ok(InstructionOp::Output(device))
            },
            38 => {
                let device = InstructionDevice::from_f_field(f_field)?;
                
                Ok(InstructionOp::JumpIfReady(device))
            },
            39 => match f_field {
                0 => Ok(InstructionOp::Jump),
                1 => Ok(InstructionOp::JumpSaveJump),
                2 => Ok(InstructionOp::JumpIfOverflow),
                3 => Ok(InstructionOp::JumpIfNoOverflow),
                _ => {
                    let flag_condition = FlagCondition::from_f_field(f_field)?;

                    Ok(InstructionOp::JumpIfFlag(flag_condition))
                }
            },
            40..48 => {
                let register = Register::from_n(c_field - 40);
                let condition = RegisterCondition::from_f_field(f_field)?;

                Ok(InstructionOp::JumpIfReg(register, condition))
            },
            48..56 => {
                let register = Register::from_n(c_field - 48);

                match f_field {
                    0 => Ok(InstructionOp::IncReg(register)),
                    1 => Ok(InstructionOp::DecReg(register)),
                    2 => Ok(InstructionOp::EnterReg(register)),
                    3 => Ok(InstructionOp::EnterRegNeg(register)),
                    _ => Err(InvalidInstruction)
                }
            }
            56..64 => {
                let register = Register::from_n(c_field - 56);

                Ok(InstructionOp::CompareReg(register))
            },
            _ => Err(InvalidInstruction)
        }
    }
}

enum ExecutionError {
    BadIndexing
}

impl Instruction {
    pub fn from_word(word: &Word) -> Result<Instruction, InvalidInstruction> {
        let op = InstructionOp::from_word(word)?;

        let f = u32::try_from(word.f_field().to_number()).unwrap();
        let i = u32::try_from(word.i_field().to_number()).unwrap();
        let a = u32::try_from(word.a_field().to_number()).unwrap();

        if i > 6 {
            return Err(InvalidInstruction);
        }

        Ok(Instruction {
            op,
            f,
            i,
            a
        })
    }

    fn index_address(&self, machine_state: &MachineState) -> i32 {
        match self.i {
            0 => i32::try_from(self.a).unwrap(),
            1 => i32::try_from(self.a).unwrap() + machine_state.registers.i1.to_number(),
            2 => i32::try_from(self.a).unwrap() + machine_state.registers.i2.to_number(),
            3 => i32::try_from(self.a).unwrap() + machine_state.registers.i3.to_number(),
            4 => i32::try_from(self.a).unwrap() + machine_state.registers.i4.to_number(),
            5 => i32::try_from(self.a).unwrap() + machine_state.registers.i5.to_number(),
            6 => i32::try_from(self.a).unwrap() + machine_state.registers.i6.to_number(),
            _ => panic!()
        }
    }

    fn load_address(&self,machine_state: &MachineState) -> Result<Word, ExecutionError> {
        
    }

    pub fn execute(&self, machine_state: &mut MachineState) -> Result<(), ExecutionError> {
        let m = self.index_address(&machine_state);

        if m < 0 {
            return Err(ExecutionError::BadIndexing);
        }

        match self.op {
            InstructionOp::NoOperation => Ok(()),
            InstructionOp::Add => {
                let result = 
            }
        }
    }
}
