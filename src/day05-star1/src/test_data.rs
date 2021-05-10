use std::collections::HashMap;
use std::fs;
use std::io;

// this implementation has got to be pretty bad...
pub fn read_test_data() -> Result<Vec<String>, io::Error> {
    let path = fs::canonicalize("../../input-data/day05-star1/input.txt")?;
    let s = std::fs::read_to_string(path)?;

    let mut mv: Vec<String> = Vec::new();

    for line in s.lines() {
        mv.push(line.trim().to_string());
    }

    Ok(mv)
}
