// use std::collections::HashSet;
// use std::convert::TryFrom;

// #[path = "test_data.rs"]
// mod test_data;

// ////////////////////////////////////////////////////////////////////////////////


// #[derive(Debug)]
// pub enum RuntimeError {
//     LoopDetected,
//     InvalidInstructionIndex,
// }

// #[derive(Debug)]
// pub struct Processor {
//     instructions: Vec<Instruction>,
//     pub accumulator: i32,
// }

// #[derive(Debug)]
// pub struct JumpState {
//     index: usize,
//     accumulator: i32,
// }

// impl Processor {
//     pub fn new(instructions: Vec<Instruction>) -> Self {
//         Processor {
//             instructions,
//             accumulator: 0,
//         }
//     }


//     // execute(Instruction)
//     // side effects:
//     // * change accumulator
//     // * change program counter

//     // questions:
//     // * to mutate an instruction, in place or copy on write?

//     // does the empty program terminate

//     pub fn find_fatal_jump(&mut self) -> Result<(), RuntimeError> {
//         let mut index = 0;
//         let mut visited = HashSet::new();
//         let mut jumps:Vec<JumpState> = Vec::new();

//         loop {
//             if index > self.instructions.len() {
//                 break;
//             }

//             if visited.contains(&index) {
//                 let last_jump = jumps.pop().unwrap();
//                 let instruction = Instruction { operator: Operator::NoOp, operand: 0 };

//                 self.accumulator = last_jump.accumulator;
//                 index = last_jump.index;

//                 self.instructions.insert(last_jump.index, instruction);
//             } else {
//                 visited.insert(index);
//             }

//             if let Some(instruction) = self.instructions.get(index) {
//                 match instruction.operator {
//                     Operator::Acc => {
//                         self.accumulator += instruction.operand;
//                         index += 1
//                     }
//                     Operator::Jmp => {
//                         jumps.push(JumpState {
//                             index,
//                             accumulator: self.accumulator,
//                         });

//                         let mut shunt: i32 = match i32::try_from(index) {
//                             Ok(positive_number) => positive_number,
//                             Err(_) => return Err(RuntimeError::InvalidInstructionIndex),
//                         };

//                         shunt += instruction.operand;

//                         match usize::try_from(shunt) {
//                             Ok(positive_number) => index = positive_number,
//                             Err(_) => return Err(RuntimeError::InvalidInstructionIndex),
//                         }
//                     }
//                     Operator::NoOp => {
//                         index += 1
//                     }
//                 };
//             } else {
//                 return Err(RuntimeError::InvalidInstructionIndex);
//             }
//         }

//         Ok(())
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// impl Instruction {
//     pub fn parse(lines: &[String]) -> Result<Option<Vec<Instruction>>, String> {
//         if lines.is_empty() {
//             return Ok(None);
//         }

//         let mut v = Vec::new();

//         for line in lines {
//             let pieces: Vec<&str> = line.split(' ').collect();

//             let operator = match pieces.get(0) {
//                 Some(&"nop") => Operator::NoOp,
//                 Some(&"acc") => Operator::Acc,
//                 Some(&"jmp") => Operator::Jmp,
//                 _ => return Err(format!("{} is bad", line)),
//             };

//             let operand = match pieces.get(1) {
//                 Some(s) => match s.parse::<i32>() {
//                     Ok(i) => i,
//                     Err(e) => return Err(format!("{} well there's no parsing that operand", e)),
//                 },
//                 None => return Err(format!("{} has no operand?", line)),
//             };

//             v.push(Instruction { operator, operand });
//         }

//         Ok(Some(v))
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_find_fatal_jump() {
//         let lines = test_data::read_test_data("day08-star1/micro.txt").unwrap();
//         let data = Instruction::parse(&lines).unwrap();

//         if let Some(instructions) = data {
//             let mut processor = Processor::new(instructions);
//             let cliff = processor.find_fatal_jump();

//             assert_matches!(cliff, Ok(_));
//             assert_matches!(processor.accumulator, 8);
//         }
//     }

//     #[test]
//     fn test_parse() {
//         let lines = test_data::read_test_data("day08-star1/micro.txt").unwrap();
//         let data = Instruction::parse(&lines);

//         assert_matches!(data, Ok(_));

//         if let Ok(result) = data {
//             assert_matches!(result, Some(_));

//             if let Some(instructions) = result {
//                 assert_matches!(instructions.get(0), Some(_));

//                 if let Some(inst) = instructions.get(0) {
//                     assert_matches!(inst.operator, Operator::NoOp);
//                     assert_eq!(inst.operand, 0);
//                 }
//             }
//         }
//     }
// }
