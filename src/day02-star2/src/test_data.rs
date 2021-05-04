use std::fs;
use std::io;

// this implementation has got to be pretty bad...
pub fn read_test_data() -> Result<Vec<String>, io::Error> {
    let path = fs::canonicalize("../../input-data/day02-star1/passwords.txt")?;
    let s = std::fs::read_to_string(path)?;

    let mut mv: Vec<String> = Vec::new();

    for line in s.lines() {
        mv.push(line.trim().to_string());
    }

    Ok(mv)
}
