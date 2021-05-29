#[macro_use] extern crate assert_matches;

use std::fs;
use std::io;

pub fn run() {
    let mut joltages = read_test_data("day10-star1/largest.txt").unwrap();
}

fn count_breakables(adapters: &[u32]) -> Option<u32> {
    if adapters.is_empty() {
        return None
    }

    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakable_edge_cases() {
        let empty = vec![];
        assert_matches!(count_breakables(&empty), None);
    }
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
