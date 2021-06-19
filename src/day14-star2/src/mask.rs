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
    AsIs,
    One,
    Float,
}

impl TryFrom<char> for Operation {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Operation::AsIs),
            '1' => Ok(Operation::One),
            'X' => Ok(Operation::Float),
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
                Operation::AsIs => '0',
                Operation::One => '1',
                Operation::Float => 'X',
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

    pub fn find_floaters(&self) -> Vec<compute::Address> {
        // filter_map maps, then filters? or am I missing something?
        self.mask
            .iter()
            .filter(|(_, v)| &Operation::Float == *v)
            .map(|(k, _)| *k)
            .collect()
    }

    pub fn produce(&self) -> Vec<compute::Address> {
        let mut addresses = vec![];

        // for (k, v) in &self.mask {
        //     let new_bit = match v {
        //         Operation::One => 1,
        //         Operation::Zero => 0,
        //         Operation::AsIs => compute::nth_bit(input, *k),
        //     };

        //     match new_bit {
        //         1 => new_val |= 1 << k,
        //         0 => {
        //             let a = !new_val;
        //             let b = a | 1 << k;
        //             let c = !b;
        //             new_val = c;
        //         }
        //         _ => panic!("your base does not belong to us"),
        //     }
        // }

        addresses
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
        assert_eq!(Ok(Operation::Float), Operation::try_from('X'));
        assert_eq!(Ok(Operation::AsIs), Operation::try_from('0'));
        assert_eq!(Err("unrecognized char!"), Operation::try_from('?'));
    }

    #[test]
    fn test_mask_value() {
        assert_eq!("0", format!("{}", Operation::AsIs));
        assert_eq!("1", format!("{}", Operation::One));
        assert_eq!("X", format!("{}", Operation::Float));
    }

    #[test]
    fn test_mask_load() {
        let m = Mask::load("000000000000000000000000000000X1001X").unwrap();
        assert_eq!(m.mask.get(&0), Some(&Operation::Float));
        assert_eq!(m.mask.get(&1), Some(&Operation::One));
        assert_eq!(m.mask.get(&2), Some(&Operation::AsIs));
        assert_eq!(m.mask.get(&3), Some(&Operation::AsIs));
        assert_eq!(m.mask.get(&4), Some(&Operation::One));
        assert_eq!(m.mask.get(&5), Some(&Operation::Float));
        assert_eq!(m.mask.get(&7), Some(&Operation::AsIs));
    }

    #[test]
    fn test_mask_to_string() {
        let s = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let m = Mask::load(s).unwrap();
        assert_eq!(s, m.to_string());
    }

    #[test]
    fn test_find_floaters() {
        let m = Mask::load("000000000000000000000000000000X1001X").unwrap();
        assert_eq!(vec![0 , 5], m.find_floaters());
    }
}
