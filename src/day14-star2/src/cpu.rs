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

    fn update_ones(&self, a: compute::Address) -> compute::Address {
        let mut work = a;

        // probably a good case for reduce
        for bit in self.mask.as_ref().unwrap().find_oners() {
            work = compute::write_nth_bit(work, bit, true);
        }

        work
    }

    pub fn alchemy(&self, a: compute::Address) -> Vec<compute::Address> {
        // this line was painful to sweat out
        let mask = self.mask.as_ref().unwrap();
        let addr = self.update_ones(a);
        let floaters = mask.find_floaters();
        let mut new_addresses = Vec::new();

        for permutation in compute::loose_the_permutations_of_war(&floaters) {
            let floated = compute::float_bit_array(&permutation, &floaters);
            let mut new_address = addr;
            for (key, val) in floated {
                new_address = compute::write_nth_bit(new_address, key, val == 1);
            }
            new_addresses.push(new_address);
        }

        new_addresses
    }

    // yes I am stealing ownership of the instructions
    // ... something something unclear
    pub fn run(&mut self, instructions: instruction::Instructions) {
        for instruction in instructions {
            if let Some(mask) = instruction.mask {
                self.mask = Some(mask);
            } else if let Some(memset) = instruction.memset {
                let (address, value) = memset;
                for addr in self.alchemy(address) {
                    self.heap.insert(addr, value);
                }
            } else {
                panic!("bogus instruction detected!");
            }
        }
    }

    pub fn sum(&self) -> compute::Value {
        self.heap.values().sum()
    }
}
