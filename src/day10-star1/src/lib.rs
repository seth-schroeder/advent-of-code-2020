mod adapter;

use adapter::Adapter;
use std::collections::HashMap;
use std::fs;
use std::io;

pub fn run() {
    if let Ok(adapters) = make_adapters() {
        let grouped_differences = compute_differences(&adapters);
        let ones = grouped_differences[&1];
        let threes = grouped_differences[&3];
        eprintln!("{} * {} = {}", ones, threes, ones * threes);
    }
}

fn compute_differences(adapters: &[Adapter]) -> HashMap<u32, u32> {
    let mut h: HashMap<u32, u32> = HashMap::new();

    for (i, adapter) in adapters.iter().enumerate() {
        if let Some(next_adapter) = adapters.get(i + 1) {
            let difference = next_adapter.joltage - adapter.joltage;

            *h.entry(difference).or_insert(0) += 1;
        }
    }

    h
}

fn make_adapters() -> Result<Vec<Adapter>, ()> {
    let mut joltages = read_test_data("day10-star1/largest.txt").unwrap();
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
