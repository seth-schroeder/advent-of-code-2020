use std::collections::HashMap;

pub fn next_multiple_after(target: i32, multiple: i32) -> i32 {
    target / multiple * multiple + multiple
}

pub fn delta_hash(target: i32, multiples: &[i32]) -> HashMap<i32, i32> {
    let mut h = HashMap::with_capacity(multiples.len());

    for multiple in multiples {
        let delta = next_multiple_after(target, *multiple) - target;
        if h.contains_key(&delta) {
            println!("already tracking a {:?}", delta);
        }
        h.insert(delta, *multiple);
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_multiple_after() {
        assert_eq!(15, next_multiple_after(12, 5));
    }

    #[test]
    fn test_delta_hash() {
        let target = 12;
        let multiples = vec![11, 5];
        let h = delta_hash(target, &multiples);
        println!("{:#?}", h);
        assert_eq!(h.get(&3).unwrap(), &5);
        assert_eq!(h.get(&10).unwrap(), &11);
    }
}
