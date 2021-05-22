use std::collections::HashSet;
use std::convert::TryFrom;

#[path = "test_data.rs"]
mod test_data;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Operator {
    Acc,
    Jmp,
    NoOp,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum RuntimeError {
    LoopDetected,
    InvalidInstructionIndex,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Instruction {
    operator: Operator,
    operand: i32,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct JumpState {
    index: usize,
    accumulator: Accumulator,
}

impl Instruction {
    pub fn parse(lines: &[String]) -> Result<Option<Vec<Instruction>>, String> {
        if lines.is_empty() {
            return Ok(None);
        }

        let mut v = Vec::new();

        for line in lines {
            let pieces: Vec<&str> = line.split(' ').collect();

            let operator = match pieces.get(0) {
                Some(&"nop") => Operator::NoOp,
                Some(&"acc") => Operator::Acc,
                Some(&"jmp") => Operator::Jmp,
                _ => return Err(format!("{} is bad", line)),
            };

            let operand = match pieces.get(1) {
                Some(s) => match s.parse::<i32>() {
                    Ok(i) => i,
                    Err(e) => return Err(format!("{} well there's no parsing that operand", e)),
                },
                None => return Err(format!("{} has no operand?", line)),
            };

            v.push(Instruction { operator, operand });
        }

        Ok(Some(v))
    }
}

////////////////////////////////////////////////////////////////////////////////

pub type Program = Vec<Instruction>;

////////////////////////////////////////////////////////////////////////////////

type Accumulator = i32;

pub struct Processor {
    accumulator: Accumulator,
    instruction_pointer: usize,
    steps: usize,
    jumps: Vec<JumpState>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            // this can be changed by jmp
            instruction_pointer: 0,

            // this increases monotonically
            steps: 0,

            accumulator: 0,

            jumps: Vec::new(),
        }
    }

    fn trace(&mut self, instruction: &Instruction) -> () {
        // this is super gross, but simpler than implementing Display
        // (shame on me)
        let inst = match instruction.operator {
            Operator::Acc => "acc",
            Operator::Jmp => "jmp",
            Operator::NoOp => "noop",
        };

        if self.steps == 0 {
            eprintln!("stp ip  opc  ope acc");
            eprintln!("--- --- ---- --- ---");
        }

        eprintln!(
            "{step:>0pwidth$}\t{ptr:>0pwidth$}\t{inst:<iwidth$} {operand:>pwidth$}\t {acc:<iwidth$}",
            step = self.steps,
            ptr = self.instruction_pointer,
            pwidth = 3,
            inst = inst,
            iwidth = 4,
            operand = instruction.operand,
            acc = self.accumulator
        );
    }

    pub fn run(&mut self, program: Program) -> Result<Option<Accumulator>, RuntimeError> {
        let mut visited = HashSet::new();

        loop {
            if visited.contains(&self.instruction_pointer) {
                return Err(RuntimeError::LoopDetected);
            } else {
                visited.insert(self.instruction_pointer);
            }

            if let Some(instruction) = program.get(self.instruction_pointer) {
                match instruction.operator {
                    Operator::Acc => {
                        self.trace(instruction);
                        self.accumulate(instruction.operand);
                        self.instruction_pointer += 1;
                    }
                    Operator::Jmp => {
                        self.trace(instruction);
                        self.jump(instruction.operand)?;
                    }
                    Operator::NoOp => {
                        self.trace(instruction);
                        self.instruction_pointer += 1;
                    }
                };
            } else {
                break;
            }
            self.steps += 1
        }

        Ok(Some(self.accumulator))
    }

    fn jump(&mut self, distance: i32) -> Result<(), RuntimeError> {
        if distance == 0 {
            return Err(RuntimeError::LoopDetected);
        }

        self.jumps.push(JumpState {
            index: self.instruction_pointer,
            accumulator: self.accumulator,
        });

        match i32::try_from(self.instruction_pointer) {
            Ok(from_usize) => match usize::try_from(from_usize + distance) {
                Ok(from_i32) => {
                    self.instruction_pointer = from_i32;
                    Ok(())
                }
                Err(_) => Err(RuntimeError::InvalidInstructionIndex),
            },
            Err(_) => Err(RuntimeError::InvalidInstructionIndex),
        }
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
        let mut processor = Processor::new();
        let result = processor.run(Program::new());

        assert_matches!(result, Ok(_));
        if let Some(number) = result.unwrap() {
            assert_eq!(0, number);
        }

        assert_eq!(0, processor.instruction_pointer);
    }

    #[test]
    fn test_noop_completes() {
        let program = vec![Instruction {
            operator: Operator::NoOp,
            operand: 0,
        }];
        let mut processor = Processor::new();
        let result = processor.run(program);

        assert_matches!(result, Ok(_));
        if let Some(number) = result.unwrap() {
            assert_eq!(0, number);
        }

        assert_eq!(1, processor.instruction_pointer);
    }

    #[test]
    fn test_acc_completes() {
        let program = vec![Instruction {
            operator: Operator::Acc,
            operand: 42,
        }];
        let mut processor = Processor::new();
        let result = processor.run(program);

        assert_matches!(result, Ok(_));
        if let Some(number) = result.unwrap() {
            assert_eq!(42, number);
        }

        assert_eq!(1, processor.instruction_pointer);
    }

    #[test]
    fn test_jmp_completes() {
        let program = vec![Instruction {
            operator: Operator::Jmp,
            operand: 1,
        }];
        let mut processor = Processor::new();
        let result = processor.run(program);

        assert_matches!(result, Ok(_));
        if let Some(number) = result.unwrap() {
            assert_eq!(0, number);
        }

        assert_eq!(1, processor.instruction_pointer);
    }

    #[test]
    fn test_loop_halts() {
        let lines = test_data::read_test_data("day08-star1/micro.txt").unwrap();

        let data = Instruction::parse(&lines);
        assert_matches!(data, Ok(_));

        let mut processor = Processor::new();

        if let Some(instructions) = data.unwrap() {
            let result = processor.run(instructions);
            assert_matches!(result, Err(RuntimeError::LoopDetected));

            assert_eq!(5, processor.accumulator);
            assert_eq!(1, processor.instruction_pointer);
        }
    }

    #[test]
    fn test_jump_state() {
        let lines = test_data::read_test_data("day08-star1/micro.txt").unwrap();

        let data = Instruction::parse(&lines);
        assert_matches!(data, Ok(_));

        let mut processor = Processor::new();

        if let Some(instructions) = data.unwrap() {
            let result = processor.run(instructions);
            assert_matches!(result, Err(RuntimeError::LoopDetected));

            assert_eq!(3, processor.jumps.len());
            assert_eq!(Some(JumpState { index: 4, accumulator: 5 }), processor.jumps.pop());
        }
    }
}
