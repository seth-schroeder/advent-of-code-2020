use regex::Regex;
use std::collections::HashMap;

pub struct Result {
    source: String,
    edges: HashMap<String, usize>,
}

pub fn parse(lines: &[String]) -> Vec<Result> {
    let mut v = Vec::new();

    for line in lines {
        let (source, raw_edges) = first_pass(line);
        let edges = second_pass(&raw_edges);
        let result = Result { source, edges };

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
