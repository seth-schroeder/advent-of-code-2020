mod adapter;
use adapter::Adapter;
use petgraph::algo;
use petgraph::graphmap::DiGraphMap;
use std::fs;
use std::io;

type AdapterGraph = DiGraphMap<u32, u32>;
type AdapterPath = Vec<u32>;

pub fn run() {
    if let Ok(adapters) = make_adapters() {
        let g = connect_adapters(&adapters);

        mess_with_topo(&g);
        println!("well, how about {}", jump(&g, &adapters));
        let ways = algo::all_simple_paths(g, 0, to: G::NodeId, min_intermediate_nodes: usize, max_intermediate_nodes: Option<usize>, )
    }
}

fn dive(g: &AdapterGraph, v: AdapterPath, node: u32) {
    // for edge in g.edges(node).iter() {
    // }
    0
}

fn jump(g: &AdapterGraph, adapters: &[Adapter]) -> u32 {
    let mut v = AdapterPath::new();
    dive(g, v, 0);
    0
}

fn mess_with_topo(g: &AdapterGraph) {
    let topo = algo::toposort(&g, None);
    eprintln!("topo: {:#?}", topo);
}

fn connect_adapters(adapters: &[Adapter]) -> AdapterGraph {
    let mut g = DiGraphMap::new();

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

            if !g.contains_edge(smaller_joltage, bigger_joltage) {
                // eprintln!(
                //     "adding edge: {:?} -> {:?} ({})",
                //     smaller_joltage, bigger_joltage, difference
                // );
                g.add_edge(smaller_joltage, bigger_joltage, difference);
            }
        }
    }

    g
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
