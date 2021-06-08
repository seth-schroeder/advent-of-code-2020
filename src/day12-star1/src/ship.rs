use crate::compass;
use crate::compute;
use std::convert::TryFrom;

type Position = (i32, i32);

#[derive(Debug)]
pub struct Ship {
    compass: compass::Compass,
    position: Position,
}

impl Ship {
    pub fn new() -> Ship {
        let mut ship = Ship {
            compass: compass::Compass::new(),
            position: (0, 0),
        };

        ship.compass.rotate(90);
        ship
    }

    pub fn fly(&mut self, instructions: &[compute::Instruction]) {
        for instruction in instructions {
            println!("fly: {:?}", instruction);
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
                    let i = compute::Instruction {
                        opcode: self.map_bearing_to_opcode(),
                        operand: instruction.operand,
                    };
                    let v = vec![i];
                    self.fly(&v);
                }
                compute::Opcode::TurnLeft => {
                    self.compass
                        .rotate(i16::try_from(-instruction.operand).unwrap());
                }
                compute::Opcode::TurnRight => {
                    self.compass
                        .rotate(i16::try_from(instruction.operand).unwrap());
                }
            }
            println!("{:#?}", self);
        }
    }

    pub fn manhattan_distance(&self) -> u32 {
        println!("ship shape: {:#?}", self);

        u32::try_from(self.position.0.abs() + self.position.1.abs()).unwrap()
    }

    fn map_bearing_to_opcode(&self) -> compute::Opcode {
        match self.compass.bearing() {
            compass::Bearing::North => compute::Opcode::MoveNorth,
            compass::Bearing::East => compute::Opcode::MoveEast,
            compass::Bearing::South => compute::Opcode::MoveSouth,
            compass::Bearing::West => compute::Opcode::MoveWest,
        }
    }
}
