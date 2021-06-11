use std::error::Error;
use std::fs;
use std::io;

mod compute;
mod ship;

pub fn run() -> Result<(), Box<dyn Error>> {
    let lines = read_test_data("day12-star1/actual.txt")?;

    let mut instructions = vec![];
    for line in lines {
        instructions.push(compute::Instruction::parse(&line).unwrap());
    }

    let mut ship = ship::Ship::new();
    ship.fly(&instructions);

    println!("ze distance might be {}", ship.manhattan_distance());
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
