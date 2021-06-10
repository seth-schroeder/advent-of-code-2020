use crate::compass;
use crate::compute;

pub struct Ship {
    pub at: compute::Point,
    pub waypoint: compute::Point,
    pub compass: compass::Compass,
}

impl Ship {
    pub fn new() -> Self {
        let mut compass = compass::Compass::new();
        compass.rotate(90);

        Ship {
            at: (0, 0),
            waypoint: (10, 1),
            compass,
        }
    }

    pub fn fly(&self, instructions: &[compute::Instruction]) {
        for instruction in instructions {
            match instruction.opcode {
                compute::Opcode::MoveNorth => (),
                compute::Opcode::MoveEast => (),
                compute::Opcode::MoveSouth => (),
                compute::Opcode::MoveWest => (),
                compute::Opcode::MoveForward => (),
                compute::Opcode::TurnLeft => (),
                compute::Opcode::TurnRight => (),
            }
        }
    }

    pub fn manhattan_distance(&self) -> u32 { 0 }

    pub fn waypoint_gap(&self) -> compute::Point {
        (self.waypoint.0.abs(), self.waypoint.1.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obligatory() {
        assert_eq!(true, !false);
    }
}
