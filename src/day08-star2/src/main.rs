#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod processor;

use processor::{Accumulator, Instruction, JumpState, Processor, Program, RuntimeError};
use std::error::Error;
mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data("day08-star1/micro.txt")?;
    let data: Program = Instruction::parse(&lines)?;

    run_trials(&data);

    Ok(())
}

fn run_trials(initial_program: &Program) -> Accumulator {
    let mut processor = Processor::new();

    if let Some(value) = run_trial(&mut processor, &initial_program) {
        return value;
    }

    0
}

fn run_trial(processor: &mut Processor, program: &Program) -> Option<Accumulator> {
    match processor.run(program) {
        Ok(_) => Some(processor.accumulator),
        Err(RuntimeError::LoopDetected) => None,
        Err(RuntimeError::InvalidInstructionIndex) => panic!("yo?"),
    }
}
