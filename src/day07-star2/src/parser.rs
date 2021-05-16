use petgraph::graph::{Graph};
use petgraph::graphmap::DiGraphMap;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawData {
    source: String,
    edges: HashMap<String, usize>,
}

impl RawData {
    pub fn graph(results: &[RawData]) -> Graph<&String, usize> {
        let mut g = Graph::new();

        for result in results {
            let source = g.add_node(&result.source);

            for (destination, weight) in &result.edges {
                let destination = g.add_node(destination);

                g.add_edge(source, destination, *weight);
            }
        }

        g
    }

    pub fn trigraph(results: &Vec<RawData>) -> DiGraphMap<&str, usize> {
        let mut v = DiGraphMap::new();

        for result in results {
            for (destination, weight) in &result.edges {
                v.add_edge(&result.source[..], destination, *weight);
            }
        }

        v
    }

    pub fn digraph(results: &[RawData]) -> DiGraphMap<&String, usize> {
        let mut g = DiGraphMap::new();

        for result in results {
            for (destination, weight) in &result.edges {
                g.add_edge(&result.source, destination, *weight);
            }
        }

        g
    }
}

pub fn parse(lines: &[String]) -> Vec<RawData> {
    let mut v = Vec::new();

    for line in lines {
        let (source, raw_edges) = first_pass(line);
        let edges = second_pass(&raw_edges);
        let result = RawData { source, edges };

        v.push(result);
    }

    v
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

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = vec![
            String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            String::from("faded blue bags contain no other bags."),
        ];

        let mut results = parse(&lines);

        let result = results.pop().unwrap();
        assert_eq!("faded blue", result.source);
        assert!(result.edges.is_empty());

        let result = results.pop().unwrap();
        assert_eq!("light red", result.source);
        assert_eq!(Some(&1_usize), result.edges.get("bright white"));
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
