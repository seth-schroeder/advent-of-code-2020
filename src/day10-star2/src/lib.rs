#[macro_use]
extern crate assert_matches;

use num_integer;
use permutator::HeapPermutationIterator;
use permutator::{CartesianProduct, Combination, Permutation};
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;
use std::fs;
use std::io;

type Adapters = Vec<u32>;

pub fn run() {
    // let v = prompt_for_test_data();
    let v = read_test_data("day10-star1/smallest.txt").unwrap();
    let h = eighty_third_attempt(&v);
    let k = consecutive_integer_keys(&h);
    let group_sizes: Vec<u32> = k.iter().map(|g| u32::try_from(g.1).unwrap()).collect();
    let breakable_group_contents: Vec<u32> = group_sizes
        .iter()
        .map(|g| if g < &3 { 0 } else { *g - 2 })
        .collect();

    println!("Working with {:?}", h);
    println!("and the keys? -> {:#?}", k);
    println!("group_sizes => {:?}", group_sizes);
    println!("breakable_group_contents => {:?}", breakable_group_contents);

    println!(
        "ze magic 010 ball says = {}",
        mind_the_ps_qs_ns_and_ks(&breakable_group_contents)
    );
}

#[derive(Debug, Clone)]
struct Coefficient {
    n: u32,
    k: u32,
}

impl Coefficient {
    fn value(&self) -> u32 {
        num_integer::binomial(self.n, self.k)
    }
}

fn generate_coefficients(n: u32) -> Vec<Coefficient> {
    let mut coefficients = Vec::new();
    let mut k = n;

    // panic when n == 0?

    loop {
        coefficients.push(Coefficient { n, k });

        if k == 0 {
            break;
        }
        k -= 1;
    }

    coefficients
}

fn mind_the_ps_qs_ns_and_ks(adapters: &Adapters) -> u32 {
    let mut coefficients = Vec::new();

    for adapter in adapters {
        if *adapter < 1 {
            continue;
        }

        coefficients.push(generate_coefficients(*adapter));
    }
    println!("coefficients: {:?}", coefficients);

    let mut sum = 0;

    // let data = &mut [1,2,3];
    // let mut slices = Vec::new();
    // coefficients.into_iter().for_each(|c| {
    //     slices.push(&coefficient);
    // }

    // compiles, does not work
    permutator::cartesian_product(&[&coefficients[..]], |product| {
        println!("product = {:?}", product);
    });
    permutator::cartesian_product(&[coefficients.as_slice()], |product| {
        println!("as_s = {:?}", product);
    });

    // works, not dynamic
    permutator::cartesian_product(
        &[
            &[
                Coefficient { n: 2, k: 2 },
                Coefficient { n: 2, k: 1 },
                Coefficient { n: 2, k: 0 },
            ],
            &[Coefficient { n: 1, k: 1 }, Coefficient { n: 1, k: 0 }],
        ],
        |product| {
            println!("iproduct = {:?}", product);
        },
    );

    // works, more dynamic
    permutator::cartesian_product(
        &[&coefficients.get(0).unwrap(), &coefficients.last().unwrap()],
        |p| {
            println!("eh {:?}", p);
        },
    );

    // permutator::cartesian_product(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]], |product| {
    //     println!("{:?}", product);
    // });

    // let cart = permutator::CartesianProductIterator::new(&[&coefficients[..]]);
    // // println!("zoikes: {:?}", &coefficients[..]);

    // for p in cart {
    //     println!("hmm: {:?}", p);
    // }

    sum
}

fn eighty_third_attempt(adapters: &Adapters) -> BTreeMap<usize, bool> {
    let mut h: BTreeMap<usize, bool> = BTreeMap::new();

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

fn consecutive_integer_keys(h: &BTreeMap<usize, bool>) -> Vec<(usize, usize)> {
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
