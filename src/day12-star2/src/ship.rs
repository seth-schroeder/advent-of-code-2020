use crate::compute;

#[derive(Debug)]
pub struct Ship {
    at: compute::Point,
    waypoint_at: compute::Point,
    waypoint_slope: compute::Point,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            at: (0, 0),
            waypoint_at: (10, 1),
            waypoint_slope: (10, 1),
        }
    }

    pub fn fly(&mut self, instructions: &[compute::Instruction]) {
        for instruction in instructions {
            match instruction.opcode {
                compute::Opcode::MoveNorth => {
                    self.waypoint_slope.1 += instruction.operand;
                }
                compute::Opcode::MoveEast => {
                    self.waypoint_slope.0 += instruction.operand;
                }
                compute::Opcode::MoveSouth => {
                    self.waypoint_slope.1 -= instruction.operand;
                }
                compute::Opcode::MoveWest => {
                    self.waypoint_slope.0 -= instruction.operand;
                }
                compute::Opcode::MoveForward => {
                    let delta = (
                        instruction.operand * self.waypoint_slope.0,
                        instruction.operand * self.waypoint_slope.1,
                    );

                    self.at.0 += delta.0;
                    self.at.1 += delta.1;

                    self.waypoint_at.0 += delta.0;
                    self.waypoint_at.1 += delta.1;
                }
                compute::Opcode::TurnLeft => {
                    self.waypoint_slope =
                        compute::rotate_counterclockwise(self.waypoint_slope, instruction.operand)
                            .unwrap();
                    self.waypoint_at.0 += self.waypoint_slope.0;
                    self.waypoint_at.1 += self.waypoint_slope.1;
                }
                compute::Opcode::TurnRight => {
                    self.waypoint_slope =
                        compute::rotate_clockwise(self.waypoint_slope, instruction.operand)
                            .unwrap();
                    self.waypoint_at.0 += self.waypoint_slope.0;
                    self.waypoint_at.1 += self.waypoint_slope.1;
                }
            }
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.at.0.abs() + self.at.1.abs()
    }
}
