#[derive(Clone, Copy)]
pub enum Sign {
    Pos,
    Neg,
}

const BIG_WORD_SIZE: usize = 5;
const BYTE_BASE: u32 = 64;

#[derive(Clone)]
pub struct Word {
    sign: Sign,
    bytes: Vec<u8>,
}

impl Word {
    pub fn size(&mut self) -> usize {
        self.bytes.len()
    }

    pub fn to_number(&self) -> i32 {
        let mut unsigned: u32 = 0;
        for (significance, x) in self.bytes.iter().enumerate() {
            let mult = BYTE_BASE.pow(u32::try_from(significance).unwrap());
            let x: u32 = (*x).into();
            unsigned += x * mult;
        }

        let signed: i32 = match self.sign {
            Sign::Pos => i32::try_from(unsigned).unwrap(),
            Sign::Neg => -i32::try_from(unsigned).unwrap(),
        };

        signed
    }

    pub fn from_number(number: i32, word_size: usize) -> Self {
        let sign = match number.signum() {
            -1 => Sign::Neg,
            _ => Sign::Pos,
        };

        let number: u32 = u32::try_from(number.abs()).unwrap();

        let mut bytes = Vec::with_capacity(word_size);

        for significance in 0..word_size {
            let mult = BYTE_BASE.pow(u32::try_from(significance).unwrap());
            let digit = (number / mult) % BYTE_BASE;
            bytes.push(digit as u8);
        }

        Self { sign, bytes }
    }

    pub fn c_field(&self) -> Word {
        assert_eq!(self.bytes.len(), BIG_WORD_SIZE);

        Word {
            bytes: Vec::from(&self.bytes[0..1]),
            sign: Sign::Pos,
        }
    }

    pub fn f_field(&self) -> Word {
        assert_eq!(self.bytes.len(), BIG_WORD_SIZE);

        Word {
            bytes: Vec::from(&self.bytes[1..2]),
            sign: Sign::Pos,
        }
    }

    pub fn i_field(&self) -> Word {
        assert_eq!(self.bytes.len(), BIG_WORD_SIZE);

        Word {
            bytes: Vec::from(&self.bytes[2..3]),
            sign: Sign::Pos,
        }
    }

    pub fn a_field(&self) -> Word {
        assert_eq!(self.bytes.len(), BIG_WORD_SIZE);

        Word {
            bytes: Vec::from(&self.bytes[3..5]),
            sign: Sign::Pos,
        }
    }

    pub fn subword(&self, start: u8, end: u8) -> Word {
        assert_eq!(self.bytes.len(), BIG_WORD_SIZE);

        assert!(start <= end);
        assert!(end <= 5);

        let sign = if start == 0 { self.sign } else { Sign::Pos };

        let bytes = Vec::from(
            &self.bytes[(BIG_WORD_SIZE - end as usize)..(BIG_WORD_SIZE - start as usize)],
        );

        Word { sign, bytes }
    }
}

pub struct AdditionResult {
    result: Word,
    overflow: bool,
}
impl std::ops::Add for Word {
    type Output = AdditionResult;

    fn add(self, other: Self) -> AdditionResult {
        let num_result: i32 = self.to_number() + other.to_number();

        let result = Self::from_number(num_result, BIG_WORD_SIZE + 1);

        let overflow = result.bytes[BIG_WORD_SIZE] != 0;

        AdditionResult { result, overflow }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_number() {
        let bytes = vec![1, 1, 0, 0, 0];

        let word = Word {
            bytes,
            sign: Sign::Pos,
        };

        assert_eq!(word.to_number(), 1 + BYTE_BASE as i32);

        let bytes = vec![1, 0, 4, 0, 0];
        let word = Word {
            bytes,
            sign: Sign::Neg,
        };

        assert_eq!(word.to_number(), -((1 + BYTE_BASE * BYTE_BASE * 4) as i32));
    }

    #[test]
    fn from_number() {
        let number = 110;

        let word = Word::from_number(number, 5);

        assert_eq!(
            word.bytes[0],
            u8::try_from(number - BYTE_BASE as i32).unwrap()
        );
        assert_eq!(word.bytes[1], 1);
        assert_eq!(word.bytes[2], 0);
        assert_eq!(word.bytes[3], 0);
        assert_eq!(word.bytes[4], 0);
        assert!(matches!(word.sign, Sign::Pos));
    }

    #[test]
    fn add_words() {
        let word1 = Word::from_number(110, 5);
        let word2 = Word::from_number(324, 5);

        let sum = word1 + word2;

        assert_eq!(sum.result.to_number(), 110 + 324);
        assert!(!sum.overflow);

        let word1 = Word::from_number(110, 5);
        let mut word2 = Word::from_number(324, 5);

        word2.sign = Sign::Neg;

        let sum = word1 + word2;

        assert_eq!(sum.result.to_number(), 110 - 324);
        assert!(!sum.overflow);
    }

    #[test]
    fn fields() {
        let word = Word {
            bytes: vec![51, 21, 24, 13, 23],
            sign: Sign::Neg,
        };

        assert_eq!(word.c_field().to_number(), 51);
        assert_eq!(word.f_field().to_number(), 21);
        assert_eq!(word.i_field().to_number(), 24);
        assert_eq!(word.a_field().to_number(), 13 + BYTE_BASE as i32 * 23);

        assert_eq!(word.subword(0, 5).to_number(), word.to_number());
    }
}
