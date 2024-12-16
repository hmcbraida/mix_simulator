use crate::word::Word;

pub enum IOError {
    DoesNotExist
}

pub trait IOElement {
    fn block_size() -> u8;
}

pub trait ReadElement: IOElement {
    fn read(&self) -> Result<Vec<Word>, IOError>;
}

pub trait WriteElement: IOElement {
    fn write(&mut self, data: Vec<Word>) -> Result<(), IOError>;
}

pub struct TapeUnit;

impl IOElement for TapeUnit {
    fn block_size() -> u8 {
        100
    }
}

impl ReadElement for TapeUnit {
    fn read(&self) -> Result<Vec<Word>, IOError> {
        Err(IOError::DoesNotExist)
    }
}

pub struct DrumUnit;

impl IOElement for DrumUnit {
    fn block_size() -> u8 {
        100
    }
}

impl ReadElement for DrumUnit {
    fn read(&self) -> Result<Vec<Word>, IOError> {
        Err(IOError::DoesNotExist)
    }
}

pub struct CardReadUnit;

impl IOElement for CardReadUnit {
    fn block_size() -> u8 {
        16
    }
}

impl ReadElement for CardReadUnit {
    fn read(&self) -> Result<Vec<Word>, IOError> {
        Err(IOError::DoesNotExist)
    }
}

pub struct CardWriteUnit;

impl IOElement for CardWriteUnit {
    fn block_size() -> u8 {
        100
    }
}

impl ReadElement for CardWriteUnit {
    fn read(&self) -> Result<Vec<Word>, IOError> {
        Err(IOError::DoesNotExist)
    }
}
