use crate::compute;
use crate::instruction;
use crate::mask;

pub struct Cpu {
    heap: compute::Heap,
    mask: Option<mask::Mask>,
    step: u32
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            heap: compute::Heap::new(),
            mask: None,
            step: 0
        }
    }

    fn update_ones(&self, a: compute::Address) -> compute::Address {
        let mut work = a;

        // probably a good case for reduce
        for bit in self.mask.as_ref().unwrap().find_oners() {
            work = compute::write_nth_bit(work, bit, true);
        }

        println!("u1: {:064b}\n  | 0000000000000000000000000000{:}\n  = {:064b}", a, self.mask.as_ref().unwrap().to_string(), work);
        work
    }

    fn trace(&self) {
        let step = format!("{:04x}", self.step);
        let mask = self.mask.as_ref().unwrap();
        let floaters = mask.find_floaters();
        println!("{}: {} => {:?}", step, mask, mask.find_floaters());
        for permutation in compute::loose_the_permutations_of_war(&floaters) {
            let floated = compute::float_bit_array(&permutation, &floaters);
            println!("    {:?} => {:?}", permutation, floated);
        }
    }

    pub fn alchemy(&self, a: compute::Address) -> Vec<compute::Address> {
        // this line was painful to sweat out
        let mask = self.mask.as_ref().unwrap();
        let addr = self.update_ones(a);
        let floaters = mask.find_floaters();
        let mut new_addresses = Vec::new();

        self.trace();
        for permutation in compute::loose_the_permutations_of_war(&floaters) {
            let floated = compute::float_bit_array(&permutation, &floaters);
            let mut new_address = addr;
            for (key, val) in floated {
                new_address = compute::write_nth_bit(new_address, key, val == 1);
            }
            new_addresses.push(new_address);
        }

        // println!("alchemy -- {}/{:064b} {:064b} m {} @ {:?}  => {:?}", a, a, addr, mask.to_string(), floaters, new_addresses);
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
                    let is_old = self.heap.contains_key(&addr);
                    if is_old {
                        println!("{}({}) = {} = {}", self.sum(), is_old, addr, value);
                    }
                    self.heap.insert(addr, value);
                }
            } else {
                panic!("bogus instruction detected!");
            }
            self.step += 1;
        }
    }

    pub fn sum(&self) -> compute::Value {
        self.heap.values().sum()
    }
}
