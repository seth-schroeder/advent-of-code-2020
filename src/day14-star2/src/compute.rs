use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::ops;

pub type Address = u64;
pub type Value = u64;
pub type Heap = BTreeMap<Address, Value>;

pub fn write_nth_bit(input: Value, n: Address, state: bool) -> Value {
    match state {
        true => input | 1 << n,
        false => {
            let a = !input;
            let b = a | 1 << n;
            !b
        }
    }
}

pub fn loose_the_permutations_of_war(floaters: &[Address]) -> Vec<Vec<Value>> {
    let num_bits = binary_permutations(Value::try_from(floaters.len()).unwrap());
    let mut v = Vec::with_capacity(usize::try_from(num_bits).unwrap());

    for num in binary_permutation_range(floaters) {
        let arr = value_to_bit_array(num, Value::try_from(floaters.len()).unwrap());
        v.push(arr);
    }

    v
}

fn binary_permutations(v: Value) -> Value {
    let two: Value = 2;
    two.pow(u32::try_from(v).unwrap())
}

pub fn binary_permutation_range(v: &[Value]) -> ops::Range<Value> {
    0..binary_permutations(Value::try_from(v.len()).unwrap())
}

pub fn value_to_bit_array(v: Value, num_bits: Value) -> Vec<Value> {
    let mut ba = Vec::with_capacity(usize::try_from(num_bits).unwrap());
    let mut work = v;
    let mut i = num_bits;

    while i > 0 {
        let bit = work & 1;
        ba.insert(0, bit);
        work >>= 1;
        i -= 1;
    }

    ba
}

pub fn float_bit_array(bits: &[Value], floaters: &[Value]) -> BTreeMap<Value, Value> {
    if bits.len() != floaters.len() {
        panic!("yo that's not right");
    }

    let mut h = BTreeMap::new();

    for (i, float) in floaters.iter().enumerate() {
        let bit = bits[i];
        h.insert(*float, bit);
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_loose_the_permutations_of_war() {
        dbg!(float_bit_array(
            &loose_the_permutations_of_war(&vec![2, 5])[0],
            &vec![5, 0]
        ));
        assert_eq!(
            vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]],
            loose_the_permutations_of_war(&vec![2, 5])
        );
    }

    #[test]
    fn test_value_to_bit_array() {
        assert_eq!(vec![0], value_to_bit_array(0, 1));
        assert_eq!(vec![0, 0], value_to_bit_array(0, 2));
        assert_eq!(vec![0, 0, 0], value_to_bit_array(0, 3));
        assert_eq!(vec![1], value_to_bit_array(1, 1));
        assert_eq!(vec![0, 1], value_to_bit_array(1, 2));
        assert_eq!(vec![0, 0, 1], value_to_bit_array(1, 3));
        assert_eq!(vec![1, 1, 1], value_to_bit_array(7, 3));
        assert_eq!(vec![1, 0, 1], value_to_bit_array(5, 3));
    }

    #[test]
    fn test_byte_overflow() {
        // I will _________never________ use `as` again
        assert_eq!(vec![1, 0, 0, 0, 0, 0, 0, 0, 0], value_to_bit_array(256, 9));
    }

    #[test]
    fn test_float_bit_array() {
        let h = float_bit_array(&vec![0, 1], &vec![5, 0]);
        assert_eq!(&0, h.get(&5).unwrap());
        assert_eq!(None, h.get(&4));
        assert_eq!(None, h.get(&3));
        assert_eq!(None, h.get(&2));
        assert_eq!(None, h.get(&1));
        assert_eq!(&1, h.get(&0).unwrap());
    }
}
