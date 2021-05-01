use std::fs;
use std::io;

// this implementation has got to be pretty bad...
pub fn read_test_data() -> Result<Vec<i32>, io::Error> {
    let path = fs::canonicalize("../../input-data/day01-star1/input.txt")?;
    let s = std::fs::read_to_string(path)?;

    let mut mv: Vec<i32> = Vec::new();

    for line in s.lines() {
        mv.push(line.parse().unwrap());
    }

    Ok(mv)
}
