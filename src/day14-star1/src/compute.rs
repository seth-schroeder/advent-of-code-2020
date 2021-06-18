use std::collections::HashMap;

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
}
