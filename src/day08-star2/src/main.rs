#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod parser;

use parser::{Instruction, Processor};
use std::error::Error;
mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data("ay08-star1/full.txt")?;
    let data = Instruction::parse(&lines)?;

    if let Some(instructions) = data {
        // let mut processor = Processor::new(instructions);
        // processor.run().unwrap();

        // println!("accumulated {}", processor.accumulator);
    }

    Ok(())
}
