#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Adapter {
    pub joltage: u32,
}

impl Adapter {
    pub fn make_device(adapters: &[Adapter]) -> Option<Adapter> {
        adapters.iter().max().map(|adapter| Adapter {
            joltage: adapter.joltage + 3,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
