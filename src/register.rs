use crate::word::Word;

#[derive(Clone)]
pub struct RegisterSet {
    pub a: Word,
    pub x: Word,
    pub jump: Word,
    pub i1: Word,
    pub i2: Word,
    pub i3: Word,
    pub i4: Word,
    pub i5: Word,
    pub i6: Word
}
