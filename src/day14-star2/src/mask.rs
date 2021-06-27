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
            m.mask
                .insert(u64::try_from(i).unwrap(), Operation::try_from(c)?);
        }
        Ok(m)
    }

    fn find_operations(&self, o: Operation) -> Vec<compute::Address> {
        self.mask
            .iter()
            .filter(|(_, v)| &o == *v)
            .map(|(k, _)| *k)
            .collect()
    }

    pub fn find_floaters(&self) -> Vec<compute::Address> {
        self.find_operations(Operation::Float)
    }

    // this ... function name... hurts to look at please look somewhere else
    pub fn find_oners(&self) -> Vec<compute::Address> {
        self.find_operations(Operation::One)
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>64}",
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
        let s = "0000000000000000000000000000XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let m = Mask::load(s).unwrap();
        assert_eq!(s, m.to_string());
    }

    #[test]
    fn test_find_floaters() {
        let m = Mask::load("000000000000000000000000000000X1001X").unwrap();
        assert_eq!(vec![0, 5], m.find_floaters());
    }
}
