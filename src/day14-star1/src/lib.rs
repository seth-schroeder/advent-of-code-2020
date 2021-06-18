use std::io;
use std::fs;
use std::error::Error;

mod compute;
mod instruction;
mod mask;

pub fn run() -> Result<(), Box<dyn Error>> {
    let lines = read_test_data("day14-star1/actual.txt")?;
    let instructions = instruction::Program::parse(&lines)?;
    let program = instruction::Program::new(instructions);
    Ok(())
}

fn read_test_data(relative_file_name: &str) -> Result<Vec<String>, io::Error> {
    let path = fs::canonicalize(format!("../../input-data/{}", relative_file_name))?;
    let s = fs::read_to_string(path)?;
    let mut v = Vec::new();
    for line in s.lines() {
        v.push(line.to_string());
    }
    Ok(v)
}
