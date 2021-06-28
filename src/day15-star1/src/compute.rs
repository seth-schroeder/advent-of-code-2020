use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
struct Echo {
    most_recent: Option<usize>,
    before_that: Option<usize>,
}

impl Echo {
    fn speak(&mut self, turn: usize) -> usize {
        let i = self.interval();

        self.before_that = self.most_recent;
        self.most_recent = Some(turn);

        i.or(Some(0)).unwrap()
    }

    fn interval(&self) -> Option<usize> {
        if let Some(mr) = self.most_recent {
            if let Some(bt) = self.before_that {
                Some(mr - bt)
            } else {
                None
            }
        } else {
            None
        }
    }
}

// I went to the Grand Canyon as a kid and did the echo thing.
// It was pretty cool.
type GrandCanyon = HashMap<usize, Echo>;

fn seed_canyon(seed: &[usize]) -> GrandCanyon {
    let mut h = GrandCanyon::new();
    for (s, i) in seed.iter().enumerate() {
        h.insert(
            s,
            Echo {
                most_recent: Some(*i + 1),
                before_that: None,
            },
        );
    }
    h
}

pub fn nth_number_spoken(n: usize, seed: &[usize]) -> usize {
    let mut v = Vec::new();
    let mut gc = seed_canyon(seed);

    for s in seed {
        v.push(*s);
    }

    for i in (v.len() + 1)..=n {
        let last_spoke = *v.last().unwrap();
        println!("{}: v = {:?}, last_spoke = {:?}", i, v, last_spoke);

        let to_speak = match gc.get_mut(&last_spoke) {
            Some(echo) => {
                println!("echo pre: {:?}", echo);
                let foo = echo.speak(i);
                println!("echo post: {:?}", echo);

                foo
            }
            None => {
                gc.insert(
                    last_spoke,
                    Echo {
                        most_recent: Some(last_spoke.try_into().unwrap()),
                        before_that: None,
                    },
                );
                0
            }
        };

        println!("just spoke {} after {}", to_speak, last_spoke);
        v.push(to_speak);
    }

    println!("gc = {:?}", gc);

    *v.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let v = vec![0, 3, 6];
        assert_eq!(nth_number_spoken(4, &v), 0);
        assert_eq!(nth_number_spoken(5, &v), 3);
        assert_eq!(nth_number_spoken(6, &v), 3);
        assert_eq!(nth_number_spoken(7, &v), 1);
        assert_eq!(nth_number_spoken(8, &v), 0);
        assert_eq!(nth_number_spoken(9, &v), 4);
        assert_eq!(nth_number_spoken(10, &v), 0);
    }
}
