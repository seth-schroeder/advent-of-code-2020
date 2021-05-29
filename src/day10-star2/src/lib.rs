#[macro_use]
extern crate assert_matches;

use num_integer;
use std::fs;
use std::io;

pub fn run() {
    // let mut joltages = read_test_data("day10-star1/largest.txt").unwrap();
}

const WINDOW: u32 = 3;

// now I'm really caffeinated and the dog needs to walk soon
fn fuckit_too_early_comma_just_count_the_breakables(adapters: &[u32]) -> u32 {
    // 'breakable' probably comes from picross3d
    let mut breakable = 0;

    // the wall is unbreakable, don't look before the wall lest ye suffer runtime usize conversion errors
    let mut i = 1;

    loop {
        if let Some(previous_adapter) = adapters.get(i - 1) {
            if let Some(current_adapter) = adapters.get(i) {
                if let Some(next_adapter) = adapters.get(i + 1) {
                    let gap = next_adapter - previous_adapter;
                    if gap < WINDOW {
                        eprintln!("{} is breakable: {} <- {}:{}:{}", current_adapter, gap, previous_adapter, current_adapter, next_adapter);
                        breakable += 1;
                    }
                }
            }
        } else {
            break;
        }

        i += 1;
    }

    breakable
}

fn sum_the_mfn_coefficients(n: u32) -> u32 {
    let mut v = Vec::new();
    let mut k = 0;

    while k <= n {
        v.push(num_integer::binomial(n, k));
        k += 1;
    }

    eprintln!("{:#?}", v);
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftecjctb() {
        let input = vec![];
        assert_eq!(0, fuckit_too_early_comma_just_count_the_breakables(&input));


        let mut input = read_test_data("day10-star1/smallest.txt").unwrap();
        input.sort_unstable();
        input.insert(0, 0);
        input.push(input.last().unwrap() + 3);
        let output = fuckit_too_early_comma_just_count_the_breakables(&input);
        assert_eq!(3, output);

        assert_eq!(8, sum_the_mfn_coefficients(output));

        let mut input = read_test_data("day10-star1/medium.txt").unwrap();
        input.sort_unstable();
        input.insert(0, 0);
        input.push(input.last().unwrap() + 3);
        eprintln!("input = {:?}", input);
        let output = fuckit_too_early_comma_just_count_the_breakables(&input);
        eprintln!("found {} breakables in medium", output);

        assert_eq!(19208, sum_the_mfn_coefficients(output));
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
