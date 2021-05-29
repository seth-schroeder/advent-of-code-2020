#[macro_use]
extern crate assert_matches;

use num_integer;
use std::fs;
use std::io;

type Adapters = Vec<u32>;
type Groups = Vec<u32>;

const WINDOW: u32 = 3;

pub fn run() {
    let v = prompt_for_test_data();
    let v = prep_data(&v);
    println!("Working with {:?}", v);

    let groups = group_it_up(&v);
    println!("\n seeing these groups: {:#?}, {}", groups, groups.len());
}

fn prep_data(data: &Adapters) -> Adapters {
    let mut clean_data = Adapters::with_capacity(data.len() + 2);

    clean_data.clone_from(data);
    clean_data.push(0);
    clean_data.sort_unstable();

    clean_data.push(clean_data.iter().max().unwrap() + 3);

    clean_data
}

fn group_it_up(data: &Adapters) -> Groups {
    let mut g = Groups::new();

    let mut group_min_val = 0;
    let mut last_val = 0;
    let mut idx = 0;

    loop {
        if idx >= data.len() {
            break;
        }

        let current = *data.get(idx).unwrap();
        println!(
            "cur: {}, gmv: {}, lv: {}, idx: {}, #groups: {}",
            current,
            group_min_val,
            last_val,
            idx,
            g.len()
        );

        if current == last_val + 1 {
            last_val = current;
        } else {
            println!("starting a group at {}", idx);
            if idx > 0 {
                println!("not the first group");
                let group_size = last_val - group_min_val + 1;
                g.push(group_size);
            }
            group_min_val = current;
            last_val = current;
        }

        idx += 1
    }

    // finish up the trailing group
    g.push(last_val - group_min_val + 1);

    g
}

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
                        eprintln!(
                            "{} is breakable: {} <- {}:{}:{}",
                            current_adapter, gap, previous_adapter, current_adapter, next_adapter
                        );
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

        let mut input = read_test_data("day10-star1/exp.txt").unwrap();
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
