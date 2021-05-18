use regex::Regex;
use std::collections::HashMap;

#[path = "test_data.rs"]
mod test_data;

pub type Edges = HashMap<String, usize>;
pub type ParsedData = HashMap<String, Edges>;

pub fn dive(data: &ParsedData, node: &str) -> usize {
    let mut added = 0;

    let empty = HashMap::new();
    let edges = data.get(node).unwrap_or(&empty);
    for (node, weight) in edges {
        let bags = weight + weight * dive(data, node);
        println!("{} added {} bags for the \"{}\" node", weight, bags, node);
        added += bags;
    }

    added
}

pub fn hash_parse(lines: &[String]) -> ParsedData {
    let mut h = HashMap::new();

    for line in lines {
        let (source, raw_edges) = first_pass(line);
        let edges = second_pass(&raw_edges);
        h.insert(source, edges);
    }

    h
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

    #[test]
    fn test_dive() {
        let lines = test_data::read_test_data("day07-star2/puny.txt").unwrap();
        let parsed = hash_parse(&lines);
        assert_eq!(0, dive(&parsed, "gndn"));
        assert_eq!(0, dive(&parsed, "dotted black"));
        assert_eq!(2, dive(&parsed, "light blue"));

        let lines = test_data::read_test_data("day07-star2/small.txt").unwrap();
        let parsed = hash_parse(&lines);
        assert_eq!(0, dive(&parsed, "gndn"));
        assert_eq!(0, dive(&parsed, "dotted black"));
        assert_eq!(126, dive(&parsed, "shiny gold"));

        let lines = test_data::read_test_data("day07-star2/micro.txt").unwrap();
        let parsed = hash_parse(&lines);
        assert_eq!(0, dive(&parsed, "gndn"));
        assert_eq!(0, dive(&parsed, "dotted black"));
        assert_eq!(32, dive(&parsed, "shiny gold"));
    }
}
