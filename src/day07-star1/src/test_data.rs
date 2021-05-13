use std::fs;
use std::io;

// this implementation has got to be pretty bad...
pub fn read_test_data(relative_file_name: &str) -> Result<Vec<String>, io::Error> {
    let path = fs::canonicalize(format!("../../input-data/{}", relative_file_name))?;
    let s = std::fs::read_to_string(path)?;

    let mut mv: Vec<String> = Vec::new();

    for line in s.lines() {
        mv.push(line.trim().to_string());
    }

    Ok(mv)
}
