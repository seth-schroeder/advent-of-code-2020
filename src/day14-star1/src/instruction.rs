use crate::compute;
use crate::mask;
use regex::Regex;

pub enum Type {
    SetMask,
    WriteValue,
}

pub trait Instruction {
    fn execute(&self, heap: &mut compute::Heap, mask: &mask::Mask);
}

// augh this was so close to working...
// pub trait DebugInstruction: Instruction + std::fmt::Debug {}

#[derive(Debug)]
pub struct Mask {
    mask: mask::Mask,
}

impl Instruction for Mask {
    fn execute(&self, heap: &mut compute::Heap, mask: &mask::Mask) {
        // lots of ick
        p.mask = Some(self.mask.clone());
    }
}

#[derive(Debug)]
pub struct Value {
    address: compute::Address,
    value: compute::Value,
}

impl Instruction for Value {
    fn execute(&self, p: &mut Program) {
        // this line is a sin, but how awful do I not realize?
        let cur_val = *p.heap.get(&self.address).or(Some(&0)).unwrap();
        let new_val = match &p.mask {
            Some(mask) => mask.apply(cur_val),
            None => panic!("zoikes!")
        };
        p.heap.insert(self.address, new_val);
    }
}

pub type Instructions = Vec<Box<dyn Instruction>>;

pub struct Program {
    instructions: Instructions,
    heap: compute::Heap,
    mask: Option<mask::Mask>,
}

impl Program {
    pub fn new(instructions: Instructions) -> Self {
        Program {
            instructions,
            heap: compute::Heap::new(),
            mask: None,
        }
    }

    pub fn execute(&mut self) {
        for instruction in self.instructions.iter_mut() {
            instruction.execute(self.heap, self.mask.unwrap());
        }
    }

    pub fn parse(lines: &[String]) -> Result<Instructions, String> {
        let mut instructions = Instructions::new();
        let r = Regex::new(r"^mem\[(\d+)\]$").unwrap();

        for line in lines {
            let pieces: Vec<&str> = line.split(" = ").collect();
            match &pieces[0][0..4] {
                "mask" => {
                    let m = mask::Mask::load(pieces[1])?;
                    instructions.push(Box::new(Mask { mask: m }));
                }
                "mem[" => match r.captures(pieces[0]) {
                    Some(captures) => {
                        let address = captures[1].parse::<compute::Address>().unwrap();
                        let value = pieces[1].parse().unwrap();

                        instructions.push(Box::new(Value { address, value }))
                    }
                    None => {
                        return Err(format!("had trouble parsing the mem instruction {}", line))
                    }
                },
                _ => return Err(format!("unexpected {}", pieces[0])),
            };
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
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[123] = 45678",
        ];

        let instructions = Program::parse(&lines).unwrap();
        assert_eq!(instructions.len(), 2);
    }
}
