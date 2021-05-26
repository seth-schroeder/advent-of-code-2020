use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Adapter {
    pub joltage: u32,
}

impl Adapter {
    pub fn fits(&self, rhs: &Adapter) -> bool {
        match self.cmp(rhs) {
            Ordering::Equal => panic!("I was told this would never happen."),
            Ordering::Less => false,
            Ordering::Greater => {
                if self.joltage <= 2 {
                    true
                } else {
                    rhs.joltage >= self.joltage - 3
                }
            }
        }
    }

    pub fn make_device(adapters: &[Adapter]) -> Option<Adapter> {
        match adapters.iter().max() {
            Some(adapter) => Some(Adapter {
                joltage: adapter.joltage + 3,
            }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit() {
        assert!(Adapter { joltage: 1 }.fits(&Adapter { joltage: 0 }));
        assert!(!Adapter { joltage: 0 }.fits(&Adapter { joltage: 1 }));
        assert!(Adapter { joltage: 2 }.fits(&Adapter { joltage: 1 }));
        assert!(Adapter { joltage: 2 }.fits(&Adapter { joltage: 0 }));
        assert!(Adapter { joltage: 3 }.fits(&Adapter { joltage: 2 }));
        assert!(Adapter { joltage: 3 }.fits(&Adapter { joltage: 1 }));
        assert!(Adapter { joltage: 3 }.fits(&Adapter { joltage: 0 }));
        assert!(!Adapter { joltage: 4 }.fits(&Adapter { joltage: 0 }));
        assert!(Adapter { joltage: 4 }.fits(&Adapter { joltage: 1 }));
    }

    #[test]
    fn test_make_device() {
        let mut adapters = vec![];
        for joltage in [2, 23, 5] {
            adapters.push(Adapter { joltage });
        }
        assert_eq!(26, Adapter::make_device(&adapters).unwrap().joltage);
    }

    #[test]
    fn test_sort() {
        let mut adapters = vec![];
        for joltage in [2, 23, 1, 5, 42] {
            adapters.push(Adapter { joltage });
        }

        adapters.sort();
        assert_eq!(1, adapters[0].joltage);
        assert_eq!(42, adapters[4].joltage);
    }
}
