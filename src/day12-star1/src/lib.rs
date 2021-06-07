use std::fs;
use std::io;

pub fn run() {
    let lines = read_test_data("day12-star1/smallest.txt").unwrap();
    let mut instructions = vec![];
    for line in lines {
        instructions.push(Instruction::parse(&line).unwrap());
    }
    println!("{:#?}", instructions);
}

#[derive(Debug, PartialEq)]
enum Opcode {
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

type Operand = u8;

trait U8Parseable {
    fn parse(s: &str) -> Option<Operand>;
}

impl U8Parseable for Operand {
    fn parse(s: &str) -> Option<u8> {
        match s.parse::<u8>() {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

impl Instruction {
    fn parse(line: &str) -> Option<Instruction> {
        let s = line.trim();
        let mut chars = s.chars();
        let c = chars.next().unwrap();
        let i = chars.as_str();

        let opcode = Opcode::parse(c)?;
        let operand = Operand::parse(i)?;

        Some(Instruction { opcode, operand })
    }
}

fn read_test_data(relative_file_name: &str) -> Result<Vec<String>, io::Error> {
    let path = fs::canonicalize(format!("../../input-data/{}", relative_file_name))?;
    let s = fs::read_to_string(path)?;
    let mut v = Vec::new();
    for line in s.lines() {
        v.push(line.to_string());
    }
    Ok(v)
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
