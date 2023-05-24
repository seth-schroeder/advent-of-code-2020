#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod parser;

use parser::{Instruction, Processor};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = lucio::get_input_data(8)?;
    let data = Instruction::parse(&lines)?;

    if let Some(instructions) = data {
        let mut processor = Processor::new(instructions);
        processor.run().unwrap();

        println!("accumulated {}", processor.accumulator);
    }

    Ok(())
}
