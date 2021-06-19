use crate::compute;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Mask {
    mask: BTreeMap<compute::Address, Operation>,
}

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    One,
    Zero,
    AsIs,
}

impl TryFrom<char> for Operation {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Operation::Zero),
            '1' => Ok(Operation::One),
            'X' => Ok(Operation::AsIs),
            _ => Err("unrecognized char!"),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::One => '1',
                Operation::Zero => '0',
                Operation::AsIs => 'X',
            }
        )
    }
}

impl Mask {
    fn new() -> Self {
        Mask {
            mask: BTreeMap::new(),
        }
    }

    pub fn load(s: &str) -> Result<Mask, &'static str> {
        let mut m = Mask::new();
        for (i, c) in s.chars().rev().enumerate() {
            m.mask.insert(i as u64, Operation::try_from(c)?);
        }
        Ok(m)
    }

    pub fn apply(&self, input: compute::Value) -> compute::Value {
        let mut new_val = input;

        for (k, v) in &self.mask {
            let new_bit = match v {
                Operation::One => 1,
                Operation::Zero => 0,
                Operation::AsIs => compute::nth_bit(input, *k),
            };

            match new_bit {
                1 => new_val |= 1 << k,
                0 => {
                    let a = !new_val;
                    let b = a | 1 << k;
                    let c = !b;
                    new_val = c;
                }
                _ => panic!("your base does not belong to us"),
            }
        }

        new_val
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.mask
                .values()
                .map(|v| v.to_string())
                .rev()
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_char() {
        // went back and forth on assert_matches... standing down for now
        assert_eq!(Ok(Operation::One), Operation::try_from('1'));
        assert_eq!(Ok(Operation::Zero), Operation::try_from('0'));
        assert_eq!(Ok(Operation::AsIs), Operation::try_from('X'));
        assert_eq!(Err("unrecognized char!"), Operation::try_from('?'));
    }

    #[test]
    fn test_mask_value() {
        assert_eq!("0", format!("{}", Operation::Zero));
        assert_eq!("1", format!("{}", Operation::One));
        assert_eq!("X", format!("{}", Operation::AsIs));
    }

    #[test]
    fn test_mask_load() {
        let m = Mask::load("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(m.mask.get(&0), Some(&Operation::AsIs));
        assert_eq!(m.mask.get(&1), Some(&Operation::Zero));
        assert_eq!(m.mask.get(&2), Some(&Operation::AsIs));
        assert_eq!(m.mask.get(&3), Some(&Operation::AsIs));
        assert_eq!(m.mask.get(&4), Some(&Operation::AsIs));
        assert_eq!(m.mask.get(&5), Some(&Operation::AsIs));
        assert_eq!(m.mask.get(&6), Some(&Operation::One));
        assert_eq!(m.mask.get(&7), Some(&Operation::AsIs));
    }

    #[test]
    fn test_mask_to_string() {
        let s = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let m = Mask::load(s).unwrap();
        assert_eq!(s, m.to_string());
    }

    #[test]
    fn test_mask_binary_value() {
        let s = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let m = Mask::load(s).unwrap();
        assert_eq!(73, m.apply(11));
        assert_eq!(101, m.apply(101));
        assert_eq!(64, m.apply(0));
    }
}
