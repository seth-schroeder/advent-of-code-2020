use regex::Regex;
use std::error::Error;

mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data()?;

    Ok(())
}

fn first_pass(s: &str) -> (String, String) {
    let pat = r"^(.*) bags contain (.*)\.$";
    let re = Regex::new(&pat).unwrap();
    let caps = re.captures(s).unwrap();

    (
        String::from(caps.get(1).unwrap().as_str()),
        String::from(caps.get(2).unwrap().as_str()),
    )
}

fn second_pass(s: &str) -> Vec<String> {
    let mut v = Vec::new();

    for raw in s.split(", ") {
        let chunks: Vec<&str> = raw.split_whitespace().collect();
        let bag = format!("{} {}", chunks.get(1).unwrap(), chunks.get(2).unwrap());
        v.push(bag);
    }

    v
}

fn parse(s: &str) -> (String, Vec<String>) {
    let (outer, inner) = first_pass(s);

    (outer, second_pass(&inner))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            (
                "light red".to_string(),
                vec!["bright white".to_string(), "muted yellow".to_string()]
            )
        );
    }

    #[test]
    fn test_first_pass() {
        let (outer, inner) =
            first_pass("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(outer, "light red");
        assert_eq!(inner, "1 bright white bag, 2 muted yellow bags");
    }

    #[test]
    fn test_second_pass() {
        assert_eq!(second_pass("1 bright white bag"), vec!["bright white"]);
        assert_eq!(
            second_pass("1 bright white bag, 2 muted yellow bags"),
            vec!["bright white", "muted yellow"]
        );
    }
}
