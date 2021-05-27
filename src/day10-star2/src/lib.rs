mod adapter;
use adapter::Adapter;
use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::fs;
use std::io;

type AdapterGraph = DiGraph<u32, u32>;
type JoltageNodeIndexMap = HashMap<u32, NodeIndex>;

pub fn run() {
    if let Ok(adapters) = make_adapters() {
        let (g, m) = connect_adapters(&adapters);

        println!("{}", serde_json::to_string_pretty(&g).unwrap());
        println!("{}", serde_json::to_string_pretty(&m).unwrap());


        let wall = adapters.first().expect("the wall?");
        let device = adapters.last().expect("the phone?");
        let ways = algo::all_simple_paths::<Vec<_>, _>(
            &g,
            *m.get(&wall.joltage).unwrap(),
            *m.get(&device.joltage).unwrap(),
            0,
            None,
        );
        println!("how about {} ways?", ways.count());
    }
}

fn connect_adapters(adapters: &[Adapter]) -> (AdapterGraph, JoltageNodeIndexMap) {
    let mut g = DiGraph::new();
    let mut m = JoltageNodeIndexMap::new();

    for adapter_a in adapters {
        for adapter_b in adapters {
            if adapter_a == adapter_b {
                continue;
            }

            let (smaller_joltage, bigger_joltage) = if adapter_a < adapter_b {
                (adapter_a.joltage, adapter_b.joltage)
            } else {
                (adapter_b.joltage, adapter_a.joltage)
            };

            let difference = bigger_joltage - smaller_joltage;
            if difference > 3 {
                continue;
            }

            // has to be some or_insert mojo here
            let bigger_node_index = match m.get(&bigger_joltage) {
                Some(idx) => *idx,
                None => {
                    let idx = g.add_node(bigger_joltage);
                    m.insert(bigger_joltage, idx);
                    idx
                }
            };

            // look ma, duplicate code
            // look ma, duplicate code
            let smaller_node_index = match m.get(&smaller_joltage) {
                Some(idx) => *idx,
                None => {
                    let idx = g.add_node(smaller_joltage);
                    m.insert(smaller_joltage, idx);
                    idx
                }
            };

            if !g.contains_edge(smaller_node_index, bigger_node_index) {
                eprintln!(
                    "adding edge: {:?} -> {:?} ({})",
                    smaller_joltage, bigger_joltage, difference
                );
                g.add_edge(smaller_node_index, bigger_node_index, difference);
            }
        }
    }

    (g, m)
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
