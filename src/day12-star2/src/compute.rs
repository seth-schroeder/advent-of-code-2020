pub type Point = (i32, i32);

#[derive(Debug)]
pub struct UnsupportedRotationError {}

type Rotation = (i32, i32);
// simplified from https://en.wikipedia.org/wiki/Rotation_matrix#Common_rotations
const NINETY: Rotation = (0, 1);
const ONE_EIGHTY: Rotation = (-1, 0);
const TWO_SEVENTY: Rotation = (0, -1);

pub fn rotate_counterclockwise(
    point: Point,
    degrees: i32,
) -> Result<Point, UnsupportedRotationError> {
    let rotation = degrees_to_rotation(degrees)?;
    let (x, y) = point;
    let (cos, sin) = rotation;
    let x_ = x * cos - y * sin;
    let y_ = x * sin + y * cos;

    Ok((x_, y_))
}

pub fn rotate_clockwise(point: Point, degrees: i32) -> Result<Point, UnsupportedRotationError> {
    rotate_counterclockwise(point, clockwise_to_counterclockwise(degrees))
}

fn degrees_to_rotation(degrees: i32) -> Result<Rotation, UnsupportedRotationError> {
    match degrees {
        90 => Ok(NINETY),
        180 => Ok(ONE_EIGHTY),
        270 => Ok(TWO_SEVENTY),
        _ => Err(UnsupportedRotationError {}),
    }
}

fn clockwise_to_counterclockwise(degrees: i32) -> i32 {
    let single_rotation = degrees % 360;
    360 - single_rotation
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,
    MoveForward,
    TurnLeft,
    TurnRight,
}

impl Opcode {
    fn parse(c: char) -> Option<Opcode> {
        match c {
            'N' => Some(Opcode::MoveNorth),
            'E' => Some(Opcode::MoveEast),
            'S' => Some(Opcode::MoveSouth),
            'W' => Some(Opcode::MoveWest),
            'F' => Some(Opcode::MoveForward),
            'L' => Some(Opcode::TurnLeft),
            'R' => Some(Opcode::TurnRight),
            _ => panic!("unexpected input {}", c),
        }
    }
}

type Operand = i32;

trait OperandParseable {
    fn parse(s: &str) -> Option<Operand>;
}

impl OperandParseable for Operand {
    fn parse(s: &str) -> Option<Operand> {
        match s.parse::<Operand>() {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: Operand,
}

impl Instruction {
    pub fn parse(line: &str) -> Option<Instruction> {
        let s = line.trim();
        let mut chars = s.chars();
        let c = chars.next().unwrap();
        let i = chars.as_str();

        let opcode = Opcode::parse(c)?;
        let operand = Operand::parse(i)?;

        Some(Instruction { opcode, operand })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_counterclockwise() {
        assert_eq!((-3, 5), rotate_counterclockwise((5, 3), 90).unwrap());
        assert_eq!((-5, -3), rotate_counterclockwise((5, 3), 180).unwrap());
        assert_eq!((3, -5), rotate_counterclockwise((5, 3), 270).unwrap());
    }

    #[test]
    fn test_rotate_clockwise() {
        assert_eq!((5, 3), rotate_clockwise((-3, 5), 90).unwrap());
        assert_eq!((5, 3), rotate_clockwise((-5, -3), 180).unwrap());
        assert_eq!((5, 3), rotate_clockwise((3, -5), 270).unwrap());
    }

    #[test]
    fn test_clockwise_to_counterclockwise() {
        assert_eq!(270, clockwise_to_counterclockwise(90));
        assert_eq!(180, clockwise_to_counterclockwise(180));
        assert_eq!(90, clockwise_to_counterclockwise(270));
        assert_eq!(270, clockwise_to_counterclockwise(90 + 360));
    }

    #[test]
    fn test_it() {
        let i = Instruction::parse("R90").unwrap();
        assert_eq!(
            i,
            Instruction {
                opcode: Opcode::TurnRight,
                operand: 90
            }
        );
    }
}
