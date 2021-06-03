use std::io;
use std::fs;
use std::fmt;

pub fn run() {
    println!("yo");
}

#[derive(Debug)]
enum Seat {
    Empty,
    Full,
    Floor,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Seat::Full => '#',
            Seat::Empty => 'L',
            Seat::Floor => '.'
        })
    }
}

// okay it's getting closer to time to make this into a local crate or something
fn read_test_data(relative_file_name: &str) -> Result<Vec<Vec<Seat>>, io::Error> {
    let path = fs::canonicalize(format!("../../input-data/{}", relative_file_name))?;
    let s = fs::read_to_string(path)?;

    let mut mv = Vec::new();

    for line in s.lines() {
        let mut v = Vec::new();
        for c in line.trim().chars() {
            v.push(match c {
                '#' => Seat::Full,
                'L' => Seat::Empty,
                '.' => Seat::Floor,
                _ => panic!("bad data!"),
            });
        }
        mv.push(v);
    }

    Ok(mv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let v = read_test_data("day11-star1/smallest.txt").unwrap();
        assert_eq!("L", format!("{}", v.first().unwrap().first().unwrap()));
    }
}
