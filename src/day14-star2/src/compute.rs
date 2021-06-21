use std::collections::HashMap;
use std::ops;

pub type Address = u64;
pub type Value = u64;
pub type Heap = HashMap<Address, Value>;

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

pub fn loose_the_permutations_of_war(floaters: &[Address]) -> Vec<Vec<u8>> {
    let num_bits = binary_permutations(floaters.len() as Value);
    let mut v = Vec::with_capacity(num_bits as usize);

    for num in binary_permutation_range(floaters) {
        let arr = value_to_bit_array(num, floaters.len() as Value);
        v.push(arr);
    }

    v
}

fn binary_permutations(v: Value) -> Value {
    let two: Value = 2;
    two.pow(v as u32)
}

pub fn binary_permutation_range(v: &[Value]) -> ops::Range<Value> {
    0..binary_permutations(v.len() as Value)
}

pub fn value_to_bit_array(v: Value, num_bits: Value) -> Vec<u8> {
    // eeeeeeeeeeek this smells
    if v > 2_u32.pow(num_bits as u32).into() {
        panic!("bit underflow!");
    }

    let mut ba = Vec::with_capacity(num_bits as usize);
    let mut work = v as u8;
    let mut i = num_bits;

    while i > 0 {
        let bit = work & 1;
        ba.insert(0, bit);
        work >>= 1;
        i -= 1;
    }

    ba
}

pub fn float_bit_array(bits: &[u8], floaters: &[Value]) -> HashMap<Value, Value> {
    if bits.len() != floaters.len() {
        panic!("yo that's not right");
    }

    let mut h = HashMap::new();

    for (i, float) in floaters.iter().enumerate() {
        let bit = bits[i] as Value;
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