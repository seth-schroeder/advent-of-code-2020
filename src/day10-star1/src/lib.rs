mod adapter;

use adapter::Adapter;
use std::collections::HashMap;
use std::fs;
use std::io;

pub fn run() {
    if let Ok(adapters) = make_adapters() {
        // eprintln!("{:#?}", adapters);
        eprintln!("{:#?}, {:#?}",
                  compute_differences(&adapters),
                  group_differences(&compute_differences(&adapters)));

    }
}

fn group_differences(diffs: &[u32]) -> HashMap<u32, u32> {
    let mut h: HashMap<u32, u32> = HashMap::new();

    for diff in diffs {
        *h.entry(*diff).or_insert(0) += 1;
    }

    h
}

fn compute_differences(adapters: &[Adapter]) -> Vec<u32> {
    let mut i = 0;
    let mut all_gaps = Vec::new();

    for adapter in adapters {
        eprintln!("{}: {:?}", i, adapter);

        if let Some(next_adapter) = adapters.get(i + 1) {
            let difference = next_adapter.joltage - adapter.joltage;
            eprintln!("\t{}, {:?}", difference, next_adapter);
            all_gaps.push(difference);
        }
        i += 1;
    }

    all_gaps
}

fn make_adapters() -> Result<Vec<Adapter>, ()> {
    let mut joltages = read_test_data("day10-star1/smallest.txt").unwrap();
    let mut adapters = Vec::with_capacity(joltages.len() + 2);

    // pretending the outlet is a 0 joltage device, let's see how that goes.
    joltages.push(0);
    joltages.sort_unstable();

    for joltage in joltages {
        adapters.push(Adapter { joltage });
    }

    // ... and hey, the device has-a adapter soooooo
    adapters.push(Adapter::make_device(&adapters).unwrap());

    Ok(adapters)
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
