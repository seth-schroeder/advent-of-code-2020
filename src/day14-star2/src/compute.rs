use std::collections::HashMap;
use std::ops;

pub type Address = u64;
pub type Value = u64;
pub type Heap = HashMap<Address, Value>;

pub fn nth_bit(input: Value, n: Address) -> Value {
    let bit = input >> n & 1;

    if bit > 1 {
        panic!("new bit is {}", bit);
    }

    bit
}

pub fn binary_permutations(v: Value) -> Value {
    let two: Value = 2;
    two.pow(v as u32)
}

pub fn binary_permutation_range(v: &[Value]) -> ops::Range<Value> {
    0..binary_permutations(v.len() as Value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_bit() {
        assert_eq!(0, nth_bit(0, 0));
        assert_eq!(1, nth_bit(1, 0));
        assert_eq!(0, nth_bit(2, 0));
        assert_eq!(1, nth_bit(2, 1));
    }

    #[test]
    fn test_binary_permutations() {
        assert_eq!(1, binary_permutations(0));
        assert_eq!(2, binary_permutations(1));
        assert_eq!(4, binary_permutations(2));
        assert_eq!(8, binary_permutations(3));
        assert_eq!(16, binary_permutations(4));
    }

    #[test]
    fn test_binary_permutation_range() {
        assert_eq!(0..1, binary_permutation_range(&vec![]));
        assert_eq!(0..2, binary_permutation_range(&vec![0]));
        assert_eq!(0..4, binary_permutation_range(&vec![0, 1]));
        assert_eq!(0..8, binary_permutation_range(&vec![0, 1, 2]));
        assert_eq!(0..16, binary_permutation_range(&vec![0, 1, 2, 3]));
    }
}
