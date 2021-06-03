use array2d::Array2D;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io;

pub fn run() {
    let first_round = read_test_data("day11-star1/smallest.txt").unwrap();
    let next_round = lights_camera_action(&first_round);
    println!(
        "{}\n{}",
        textify_seating_area(&first_round),
        textify_seating_area(&next_round)
    );
}

const MAX_NEIGHBORS: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Empty,
    Full,
    Floor,
}

type SeatingArea = Array2D<Seat>;

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::Full => '#',
                Seat::Empty => 'L',
                Seat::Floor => '.',
            }
        )
    }
}

impl Seat {
    // moving a new Vec out seems really expensive
    fn adjacent_indices(x_y: (usize, usize), seats: &SeatingArea) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        let (x, y) = x_y;

        // it's normal for indices to be negative for this test
        let ix = i32::try_from(x).unwrap();
        let iy = i32::try_from(y).unwrap();
        let x_min = 0.max(ix - 1);
        let y_min = 0.max(iy - 1);

        // sending in zero seats is an error I am happy to let Rust detect in this context
        let x_max = i32::try_from((seats.num_columns() - 1).min(x + 1)).unwrap();
        let y_max = i32::try_from((seats.num_rows() - 1).min(y + 1)).unwrap();

        eprintln!(
            "aji ({}, {}) => ({}, {}), ({}, {})",
            x, y, x_min, y_min, x_max, y_max
        );

        let mut at_y = y_min;
        let mut at_x = x_min;

        while at_y <= y_max {
            while at_x <= x_max {
                if (at_x, at_y) != (ix, iy) {
                    eprintln!("inserting {:?}", (at_x, at_y));
                    v.push((
                        usize::try_from(at_x).unwrap(),
                        usize::try_from(at_y).unwrap(),
                    ));
                }
                at_x += 1;
            }
            at_x = x_min;
            at_y += 1;
        }

        v
    }
}

// hello zeeeero abstraction or organization... excuse = late
fn textify_seating_area(seats: &SeatingArea) -> String {
    let mut s = Vec::new();

    for row in seats.rows_iter() {
        let mut r = Vec::new();
        for seat in row {
            r.push(format!("{}", seat));
        }
        s.push(r.join(""));
    }

    s.join("\n")
}

fn lights_camera_action(seats: &SeatingArea) -> SeatingArea {
    let mut next_round = seats.clone();
    let mut x_y = (0, 0);

    // eprintln!("{:#?}", seats);

    while (x_y).0 < seats.num_rows() {
        while (x_y).1 < seats.num_columns() {
            let neighbors = count_neighbors(x_y, seats);
            let mut seat = seats[x_y];

            match seats[x_y] {
                Seat::Full => {
                    if neighbors > MAX_NEIGHBORS {
                        seat = Seat::Empty;
                    }
                }
                Seat::Empty => {
                    if neighbors == 0 {
                        seat = Seat::Full;
                    }
                }
                Seat::Floor => (),
            }

            next_round[x_y] = seat;

            (x_y).1 += 1;
        }
        (x_y).1 = 0;
        (x_y).0 += 1;
    }

    next_round
}

fn count_neighbors(x_y: (usize, usize), seats: &SeatingArea) -> u8 {
    let mut headcount = 0;
    let to_visit = Seat::adjacent_indices(x_y, seats);

    for nx_ny in to_visit {
        if let Seat::Full = seats[nx_ny] {
            headcount += 1;
        }
    }

    headcount
}

// okay it's getting closer to time to make this into a local crate or something
fn read_test_data(relative_file_name: &str) -> Result<Array2D<Seat>, io::Error> {
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

    Ok(Array2D::from_rows(&mv))
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let v = read_test_data("day11-star1/smallest.txt").unwrap();
        println!("{:#?}", v);
        assert_eq!("L", format!("{}", v[(0, 0)]));
    }

    #[test]
    fn test_equality() {
        assert_eq!(
            read_test_data("day11-star1/smallest.txt").unwrap(),
            read_test_data("day11-star1/smallest.txt").unwrap()
        );
    }

    #[test]
    fn test_next_round() {
        let expected = vec![
            "#.##.##.##",
            "#######.##",
            "#.#.#..#..",
            "####.##.##",
            "#.##.##.##",
            "#.#####.##",
            "..#.#.....",
            "##########",
            "#.######.#",
            "#.#####.##",
        ]
        .join("\n");

        let first_round = read_test_data("day11-star1/smallest.txt").unwrap();
        let next_round = lights_camera_action(&first_round);
        assert_eq!(expected, textify_seating_area(&next_round));

        let expected = vec![
            "#.LL.L#.##",
            "#LLLLLL.L#",
            "L.L.L..L..",
            "#LLL.LL.L#",
            "#.LL.LL.LL",
            "#.LLLL#.##",
            "..L.L.....",
            "#LLLLLLLL#",
            "#.LLLLLL.L",
            "#.#LLLL.##",
        ].join("\n");

        let third_round = lights_camera_action(&next_round);
        assert_eq!(expected, textify_seating_area(&third_round));
    }
}
