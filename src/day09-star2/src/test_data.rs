use std::fs;
use std::io;

// this implementation has got to be pretty bad...
pub fn read_test_data(relative_file_name: &str) -> Result<Vec<u64>, io::Error> {
    let path = fs::canonicalize(format!("../../input-data/{}", relative_file_name))?;
    let s = fs::read_to_string(path)?;

    let mut mv: Vec<u64> = Vec::new();

    for line in s.lines() {
        mv.push(line.trim().to_string().parse().unwrap());
    }

    Ok(mv)
}
