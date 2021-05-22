#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod processor;

use processor::{Instruction, Processor, Program};
use std::error::Error;
mod test_data;

fn run_trial(program: Program) -> Program {
    program.clone()
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data("ay08-star1/micro.txt")?;
    let data: Program = Instruction::parse(&lines)?;

    Ok(())
}
