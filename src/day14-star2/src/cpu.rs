use crate::compute;
use crate::instruction;
use crate::mask;

pub struct Cpu {
    heap: compute::Heap,
    mask: Option<mask::Mask>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            heap: compute::Heap::new(),
            mask: None,
        }
    }

    // yes I am stealing ownership of the instructions
    // ... something something unclear
    pub fn run(&mut self, instructions: instruction::Instructions) {
        for instruction in instructions {
            if let Some(mask) = instruction.mask {
                self.mask = Some(mask);
            } else if let Some(memset) = instruction.memset {
                // let (address, value) = memset;
                // let output = self.mask.as_ref().unwrap().apply(value);
                // self.heap.insert(address, output);
            } else {
                panic!("bogus instruction detected!");
            }
        }
    }

    pub fn sum(&self) -> compute::Value {
        self.heap.values().sum()
    }
}
