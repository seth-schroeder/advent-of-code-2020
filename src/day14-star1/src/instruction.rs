use crate::compute;
use crate::mask;
use regex::Regex;

// augh this was so close to working...
// pub trait DebugInstruction: Instruction + std::fmt::Debug {}

pub struct Instruction {
    pub mask: Option<mask::Mask>,
    pub memset: Option<(compute::Address, compute::Value)>,
}

pub type Instructions = Vec<Instruction>;

impl Instruction {
    pub fn parse(lines: &[String]) -> Result<Instructions, String> {
        let mut instructions = Instructions::new();
        let r = Regex::new(r"^mem\[(\d+)\]$").unwrap();

        for line in lines {
            let mut mask = None;
            let mut memset = None;
            let pieces: Vec<&str> = line.split(" = ").collect();

            match &pieces[0][0..4] {
                "mask" => {
                    mask = Some(mask::Mask::load(pieces[1])?);
                }
                "mem[" => match r.captures(pieces[0]) {
                    Some(captures) => {
                        let address = captures[1].parse::<compute::Address>().unwrap();
                        let value = pieces[1].parse::<compute::Value>().unwrap();
                        memset = Some((address, value));
                    }
                    None => {
                        return Err(format!("had trouble parsing the mem instruction {}", line))
                    }
                },
                _ => return Err(format!("unexpected {}", pieces[0])),
            };

            instructions.push(Instruction { mask, memset });
        }

        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[123] = 45678".to_string(),
        ];

        let instructions = Instruction::parse(&lines[..]).unwrap();
        assert_eq!(instructions.len(), 2);
    }
}
