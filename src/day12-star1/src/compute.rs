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
