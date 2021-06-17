use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

pub type Address = u64;
pub type Value = u64;
pub type Heap = Vec<Value>;

struct Mask {
    mask: HashMap<Address, MaskOperation>,
}

// pub fn maskify(input: Value, nthBit: u8, mask: Mask) -> Value

#[derive(Debug, PartialEq)]
enum MaskOperation {
    One,
    Zero,
    AsIs,
}

impl TryFrom<char> for MaskOperation {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(MaskOperation::Zero),
            '1' => Ok(MaskOperation::One),
            'X' => Ok(MaskOperation::AsIs),
            _ => Err("unrecognized char!"),
        }
    }
}

impl fmt::Display for MaskOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MaskOperation::One => '1',
                MaskOperation::Zero => '0',
                MaskOperation::AsIs => 'X',
            }
        )
    }
}

impl Mask {
    fn new() -> Self {
        Mask {
            mask: HashMap::new(),
        }
    }

    fn load(s: &str) -> Result<Mask, &'static str> {
        let mut m = Mask::new();
        for (i, c) in s.chars().rev().enumerate() {
            m.mask.insert(i as u64, MaskOperation::try_from(c)?);
        }
        Ok(m)
    }

    fn store(&self) -> String {
        let mut s = String::new();

        s
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let mut arr = Vec::with_capacity(self.mask.len());
        write!(f, "{}", 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_char() {
        // went back and forth on assert_matches... standing down for now
        assert_eq!(Ok(MaskOperation::One), MaskOperation::try_from('1'));
        assert_eq!(Ok(MaskOperation::Zero), MaskOperation::try_from('0'));
        assert_eq!(Ok(MaskOperation::AsIs), MaskOperation::try_from('X'));
        assert_eq!(Err("unrecognized char!"), MaskOperation::try_from('?'));
    }

    #[test]
    fn test_mask_value() {
        assert_eq!("0", format!("{}", MaskOperation::Zero));
        assert_eq!("1", format!("{}", MaskOperation::One));
        assert_eq!("X", format!("{}", MaskOperation::AsIs));
    }

    #[test]
    fn test_mask_load() {
        let m = Mask::load("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(m.mask.get(&0), Some(&MaskOperation::AsIs));
        assert_eq!(m.mask.get(&1), Some(&MaskOperation::Zero));
        assert_eq!(m.mask.get(&2), Some(&MaskOperation::AsIs));
        assert_eq!(m.mask.get(&3), Some(&MaskOperation::AsIs));
        assert_eq!(m.mask.get(&4), Some(&MaskOperation::AsIs));
        assert_eq!(m.mask.get(&5), Some(&MaskOperation::AsIs));
        assert_eq!(m.mask.get(&6), Some(&MaskOperation::One));
        assert_eq!(m.mask.get(&7), Some(&MaskOperation::AsIs));
    }
}
