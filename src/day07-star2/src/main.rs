use petgraph::algo;
use petgraph::graph::Graph;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data("day07-star1/haiku.txt")?;

    // separate parsing the text from populating the graph
    let map = hash_map_that_will_almost_certainly_be_killed_by_the_second_star(lines);

    // keep track of the node names and their instances
    let mut edge_map = HashMap::new();

    let mut g: Graph<&str, i32> = Graph::new();

    // add the nodes to the graph
    for key in map.keys() {
        let node = g.add_node(key);
        edge_map.insert(key, node);
    }

    // draw the edges
    for (from_key, edges) in map.iter() {
        let from_node = edge_map.get(from_key).expect("eh");

        for to_key in edges {
            let to_node = edge_map.get(to_key).expect("bruh");
            g.add_edge(*from_node, *to_node, 0);
        }
    }

    // identify the target node
    // FIXME: need a better way to call `get`
    let target_node = edge_map.get(&"shiny gold".to_string()).unwrap();

    // using a topo sort lets us stop as soon as we run into the target node
    let topo = algo::toposort(&g, None).unwrap();

    let mut containers = 0;
    let mut visited = 0;
    println!("found {} nodes in topo", topo.len());

    for node in &topo {
        visited += 1;

        if node == target_node {
            println!("stopping after {} of {}", visited, topo.len());
            break;
        }

        if algo::has_path_connecting(&g, *node, *target_node, None) {
            println!("{:?} connects to {:?}", node, target_node);
            containers += 1;
        }
    }

    println!("There were {} containers of {:?}", containers, target_node);

    Ok(())
}

fn first_pass(s: &str) -> (String, String) {
    let pat = r"^(.*) bags contain (.*)\.$";
    let re = Regex::new(&pat).unwrap();
    let caps = match re.captures(s) {
        Some(x) => x,
        None => {
            panic!("choked on {}", s);
        }
    };

    (
        String::from(caps.get(1).unwrap().as_str()),
        String::from(caps.get(2).unwrap().as_str()),
    )
}

fn second_pass(s: &str) -> HashMap<String, usize> {
    let mut h = HashMap::new();

    for raw in s.split(", ") {
        if raw == "no other bags" {
            break;
        }

        let chunks: Vec<&str> = raw.split_whitespace().collect();

        let bag = format!("{} {}", chunks.get(1).unwrap(), chunks.get(2).unwrap());
        let count = chunks.get(0).unwrap().parse().unwrap();
        h.insert(bag, count);
    }

    h
}

fn parse(s: &str) -> (String, HashMap<String, usize>) {
    let (outer, inner) = first_pass(s);

    (outer, second_pass(&inner))
}

fn hash_map_that_will_almost_certainly_be_killed_by_the_second_star(
    v: Vec<String>,
) -> HashMap<String, Vec<String>> {
    let mut h = HashMap::new();

    for line in v {
        let (key, val) = parse(&line);
        h.insert(key, val);
    }

   HashMap::new() 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_petgraph() {}

    #[test]
    fn test_parse() {
        let mut expected = HashMap::new();

        expected.insert(String::from("bright white"), 1);
        expected.insert(String::from("muted yellow"), 2);

        assert_eq!(
            parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            (
                "light red".to_string(),
                expected
            )
        );
    }

    #[test]
    fn test_first_pass() {
        let (outer, inner) =
            first_pass("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(outer, "light red");
        assert_eq!(inner, "1 bright white bag, 2 muted yellow bags");

        let (outer, inner) = first_pass("faded blue bags contain no other bags.");
        assert_eq!(outer, "faded blue");
        assert_eq!(inner, "no other bags");
    }

    #[test]
    fn test_second_pass() {
        let mut expected = HashMap::new();

        expected.insert(String::from("bright white"), 1);
        assert_eq!(second_pass("1 bright white bag"), expected);

        expected.clear();
        expected.insert(String::from("bright white"), 1);
        expected.insert(String::from("muted yellow"), 2);

        assert_eq!(
            second_pass("1 bright white bag, 2 muted yellow bags"),
            expected
        );

        expected.clear();
        assert_eq!(second_pass("no other bags"), expected);
    }
}
