#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod processor;

use processor::{Accumulator, Instruction, Operator, Processor, Program, RuntimeError};
use std::error::Error;
mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data("day08-star1/full.txt")?;
    let data: Program = Instruction::parse(&lines)?;

    println!("ended with {}", run_trials(&data));

    Ok(())
}

fn run_trials(initial_program: &Program) -> Accumulator {
    let mut processor = Processor::new();
    let mut all_jumps = Processor::find_all_jumps(&initial_program);

    loop {
        let mut new_program = initial_program.clone();
        let last_jump = all_jumps.pop().expect("no jumps left?");

        new_program[last_jump] = Instruction {
            operator: Operator::NoOp,
            operand: -12345678,
        };

        if let Some(value) = run_trial(&mut processor, &new_program) {
            return value;
        }
    }
}

fn run_trial(processor: &mut Processor, program: &Program) -> Option<Accumulator> {
    match processor.run(program) {
        Ok(_) => Some(processor.accumulator),
        Err(RuntimeError::LoopDetected) => {
            eprintln!("*** LOOP");
            None
        }
        Err(RuntimeError::InvalidInstructionIndex) => panic!("yo?"),
    }
}
