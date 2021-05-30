#[macro_use]
extern crate assert_matches;

use num_integer;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::io;

type Adapters = Vec<u32>;

pub fn run() {
    let v = prompt_for_test_data();
    let h = eighty_third_attempt(&v);
    let k = consecutive_integer_keys(&h);

    println!("Working with {:?}", h);
    println!("and the keys? -> {:#?}", k);

    // let groups = group_it_up(&v);
    // println!("\n seeing these groups: {:?}", groups);
    // println!("\n ... and the sum of the coefficients is: {}", group_sum(&groups));
}

fn eighty_third_attempt(adapters: &Adapters) -> HashMap<usize, bool> {
    let mut h: HashMap<usize, bool> = HashMap::with_capacity(adapters.len());

    for adapter in adapters {
        h.insert(usize::try_from(*adapter).unwrap(), true);
    }

    // a month ago this line would have been a mystery.
    // now it's just a mess that wor-- has room for improvement
    let last = usize::try_from(*adapters.iter().max().unwrap()).unwrap();

    // this 1 is really strange... in the problem the adapters start at index 1.
    let mut i: usize = 1;

    while i < last {
        if !h.contains_key(&i) {
            h.insert(i, false);
        }
        i += 1;
    }

    h
}

fn consecutive_integer_keys(h: &HashMap<usize, bool>) -> Vec<(usize, usize)> {
    let mut v = Vec::new();

    let last = h.keys().max().unwrap();
    let mut i = 1; // adapters start at index 1

    let mut group_min_index = None;
    let mut group_len = 0;

    while i <= *last {
        match h.get(&i) {
            Some(true) => {
                if group_len == 0 {
                    group_min_index = Some(i);
                }

                group_len += 1;
            }
            Some(false) => {
                if group_len > 0 {
                    v.push((group_min_index.unwrap(), group_len));
                    group_len = 0;
                    group_min_index = None;
                }
            }
            None => panic!("unpossible!"),
        }

        i += 1;
    }

    // did we end in a group?
    if group_len > 0 {
        v.push((group_min_index.unwrap(), group_len));
    }

    v
}

// fn group_sum(g: &Groups) -> u32 {
//     g.iter().map(|group| sum_the_mfn_coefficients(*group)).sum()
// }

fn sum_the_mfn_coefficients(n: u32) -> u32 {
    let mut v = Vec::new();
    let mut k = 1;

    while k <= n {
        v.push(num_integer::binomial(n, k));
        k += 1;
    }

    // eprintln!("{:#?}", v);
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // struggling with the leap from node indexes to node group counts.
    // let's take it through some intermediate steps

    // first up, denormalize into contiguous boolean groups
    #[test]
    fn test_boolify() {
        let input = vec![4, 1, 3, 7];
        let output = eighty_third_attempt(&input);
        println!("{:#?}", output);

        assert_eq!(output.get(&0), Some(&false));
        assert_eq!(output.get(&1), Some(&true));
        assert_eq!(output.get(&2), Some(&false));
        assert_eq!(output.get(&3), Some(&true));
        assert_eq!(output.get(&4), Some(&true));
    }
}

// one month and still the same basic code o.O
fn read_test_data(relative_file_name: &str) -> Result<Vec<u32>, io::Error> {
    let path = fs::canonicalize(format!("../../input-data/{}", relative_file_name))?;
    let s = fs::read_to_string(path)?;

    let mut mv: Vec<u32> = Vec::new();

    for line in s.lines() {
        mv.push(line.trim().to_string().parse().unwrap());
    }

    Ok(mv)
}

fn prompt_for_test_data() -> Vec<u32> {
    let mut v = Vec::new();

    println!("> (empty line when done)");

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("need more data pls");
        let s = s.trim();

        if s.trim().is_empty() {
            break;
        } else {
            v.push(s.parse().unwrap())
        }
    }

    v
}
