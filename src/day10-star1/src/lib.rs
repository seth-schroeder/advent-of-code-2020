mod adapter;

use adapter::Adapter;
use std::collections::HashMap;

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
    let mut joltages = lucio::get_input_data(10).unwrap();
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
