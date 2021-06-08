use crate::compass::Compass;
use crate::compute;

type Position = (i32, i32);

pub struct Ship {
    compass: Compass,
    position: Position,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            compass: Compass::new(),
            position: (0, 0),
        }
    }

    fn fly(&mut self, instructions: &[compute::Instruction]) {
        for instruction in instructions {
            match instruction.opcode {
                compute::Opcode::MoveNorth => {
                    self.position.1 += instruction.operand;
                }
                compute::Opcode::MoveEast => {
                    self.position.0 += instruction.operand;
                }
                compute::Opcode::MoveSouth => {
                    self.position.1 -= instruction.operand;
                }
                compute::Opcode::MoveWest => {
                    self.position.0 -= instruction.operand;
                }
                compute::Opcode::MoveForward => {
                    // let i = Instruction {
                    //     opcode: self.orientation_to_opcode(),
                    //     operand: instruction.operand
                    // };
                    // let v = vec![i];
                    // self.fly(&v);
                    ()
                }
                compute::Opcode::TurnLeft => (),
                compute::Opcode::TurnRight => (),
            }
        }
    }

    fn manhattan_distance(&self) -> u32 {
        0
    }
}
