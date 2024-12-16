use crate::word::Word;

#[derive(Clone)]
pub struct RegisterSet {
    a: Word,
    x: Word,
    jump: Word,
    i1: Word,
    i2: Word,
    i3: Word,
    i4: Word,
    i5: Word,
    i6: Word
}
