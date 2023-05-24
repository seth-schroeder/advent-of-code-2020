use std::collections::HashSet;
use std::convert::TryFrom;

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

#[derive(Debug)]
pub enum RuntimeError {
    InvalidInstructionIndex,
}

#[derive(Debug)]
pub struct Processor {
    instructions: Vec<Instruction>,
    pub accumulator: i32,
}

impl Processor {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Processor {
            instructions,
            accumulator: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        let mut index = 0;
        let mut visited = HashSet::new();

        loop {
            if visited.contains(&index) {
                return Ok(());
            } else {
                visited.insert(index);
            }

            if let Some(instruction) = self.instructions.get(index) {
                match instruction.operator {
                    Operator::Acc => {
                        self.accumulator += instruction.operand;
                        index += 1
                    }
                    Operator::Jmp => {
                        let mut shunt: i32 = match i32::try_from(index) {
                            Ok(positive_number) => positive_number,
                            Err(_) => return Err(RuntimeError::InvalidInstructionIndex),
                        };

                        shunt += instruction.operand;

                        match usize::try_from(shunt) {
                            Ok(positive_number) => index = positive_number,
                            Err(_) => return Err(RuntimeError::InvalidInstructionIndex),
                        }
                    }
                    Operator::NoOp => index += 1,
                };
            } else {
                return Err(RuntimeError::InvalidInstructionIndex);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Instruction {
    pub fn parse(lines: &[String]) -> Result<Option<Vec<Instruction>>, String> {
        if lines.is_empty() {
            return Ok(None);
        }

        let mut v = Vec::new();

        for line in lines {
            let pieces: Vec<&str> = line.split(' ').collect();

            let operator = match pieces.first() {
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
