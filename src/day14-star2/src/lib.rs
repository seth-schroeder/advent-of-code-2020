use std::error::Error;
use std::fs;
use std::io;

mod compute;
mod cpu;
mod instruction;
mod mask;

pub fn run() -> Result<(), Box<dyn Error>> {
    // let lines = read_test_data("day14-star2/smallest.txt")?;
    let lines = read_test_data("day14-star2/smallest.txt")?;
    let instructions = instruction::Instruction::parse(&lines)?;
    let mut cpu = cpu::Cpu::new();
    cpu.run(instructions);
    println!("everything added up to be {}", cpu.sum());
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
