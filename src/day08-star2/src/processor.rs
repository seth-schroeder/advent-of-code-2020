////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Operator {
    Acc,
    Jmp,
    NoOp,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Instruction {
    operator: Operator,
    operand: i32,
}

////////////////////////////////////////////////////////////////////////////////

pub type Program = Vec<Instruction>;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ProcessingException {
    LoopDetected(String),
    InvalidInstructionIndex(String),
    UnexpectedError(String),
}

////////////////////////////////////////////////////////////////////////////////

type Accumulator = i32;

pub struct Processor {
    accumulator: Accumulator,
    instruction_pointer: usize,
    program: Program,
}

impl Processor {
    pub fn new(program: Program) -> Self {
        Processor {
            program,
            instruction_pointer: 0,
            accumulator: 0,
        }
    }

    pub fn run(&mut self) -> Result<Option<Accumulator>, ProcessingException> {
        loop {
            if let Some(instruction) = self.program.get(self.instruction_pointer) {
                match instruction.operator {
                    Operator::Acc => {
                        let operand = instruction.operand;
                        self.accumulate(operand)
                    }
                    Operator::Jmp => (),
                    Operator::NoOp => (),
                };
            } else {
                break;
            }
            // once more into the breach
            self.instruction_pointer += 1
        }

        Ok(Some(self.accumulator))
    }

    fn accumulate(&mut self, amount: Accumulator) {
        self.accumulator += amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_program_completes() {
        let result = Processor::new(Program::new()).run();

        assert_matches!(result, Ok(_));
        if let Some(number) = result.unwrap() {
            assert_eq!(0, number);
        }
    }

    #[test]
    fn test_noop_completes() {
        let program = vec![Instruction {
            operator: Operator::NoOp,
            operand: 0,
        }];
        let result = Processor::new(program).run();

        assert_matches!(result, Ok(_));
        if let Some(number) = result.unwrap() {
            assert_eq!(0, number);
        }
    }

    #[test]
    fn test_acc_completes() {
        let program = vec![Instruction {
            operator: Operator::Acc,
            operand: 42,
        }];
        let result = Processor::new(program).run();

        assert_matches!(result, Ok(_));
        if let Some(number) = result.unwrap() {
            assert_eq!(42, number);
        }
    }
}