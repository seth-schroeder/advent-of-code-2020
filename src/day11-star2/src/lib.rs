mod vector;

use array2d::Array2D;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io;

pub fn run() {
    let first_round = read_test_data("day11-star1/smallest.txt").unwrap();
    println!("{}", roll_tape(&first_round),);
}

const MAX_NEIGHBORS: u8 = 4;

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

        // "no need to unit test boundary conditions when mixing indices and counts"
        // me, several hours ago.
        let (x, y) = x_y;
        let x_min = if x == 0 { x } else { x - 1 };
        let x_max = seats.num_rows().min(x + 2);
        let y_min = if y == 0 { y } else { y - 1 };
        let y_max = seats.num_columns().min(y + 2);

        // eprintln!(
        //     "aji {}..{}..{}, {}..{}..{}",
        //     x_min, x, x_max, y_min, y, y_max
        // );

        // hello there, doubleplus extra time in Purgatory
        for x_ in x_min..x_max {
            for y_ in y_min..y_max {
                if x_y == (x_, y_) {
                    continue;
                }

                // eprintln!("aji inserting {:?} for {:?}", (x_, y_), x_y);
                v.push((x_, y_));
            }
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

// hmm the Hollywood metaphors are wan tonight
fn roll_tape(seats: &SeatingArea) -> u32 {
    let mut frames = 0;

    // pushing and popping a one element vector is a 1am workaround for juggling ownership and borrowing in a loop
    let mut v = vec![];

    loop {
        frames += 1;

        if v.is_empty() {
            v.push(lights_camera_action(seats));
            continue;
        }

        let last_area = v.first().unwrap();
        let this_area = lights_camera_action(&last_area);

        if *last_area == this_area {
            println!("\n");
            break;
        }

        print!(".");
        v.pop();
        v.push(this_area);
    }

    eprintln!("It took {} frames", frames);
    count_occupants(&v.pop().unwrap())
}

fn lights_camera_action(seats: &SeatingArea) -> SeatingArea {
    let mut next_round = seats.clone();
    let mut x_y = (0, 0);

    // eprintln!("{:#?}", seats);
    // eprintln!("===============================================");
    // eprintln!("before\n{}", textify_seating_area(seats));

    while (x_y).0 < seats.num_rows() {
        while (x_y).1 < seats.num_columns() {
            let neighbors = count_neighbors(x_y, seats);
            // eprintln!("found {} neighbors for {:?}", neighbors, x_y);
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

    eprintln!("\nafter\n{}", textify_seating_area(&next_round));
    next_round
}

fn count_occupants(seats: &SeatingArea) -> u32 {
    let mut headcount = 0;

    for row in seats.rows_iter() {
        for col in row {
            if let Seat::Full = col {
                headcount += 1;
            }
        }
    }

    headcount
}

fn count_neighbors(x_y: (usize, usize), seats: &SeatingArea) -> u8 {
    let mut headcount = 0;
    let (x, y) = x_y;
    let point = vector::Point {
        x: i32::try_from(x).unwrap(),
        y: i32::try_from(y).unwrap(),
    };
    let grid = vector::Grid {
        height: seats.num_rows(),
        width: seats.num_columns(),
    };

    for orientation in vector::Compass::rose() {
        let path = grid.path(&point, &orientation);
        let mut i = 0;
        // eprintln!("{:?} from {:?} path = {:#?}", orientation, point, path);
        while i < path.len() {
            let next_point = path.get(i).unwrap();
            let next_x_y = (
                usize::try_from(next_point.x).unwrap(),
                usize::try_from(next_point.y).unwrap(),
            );
            if seats[next_x_y] == Seat::Full {
                // eprintln!("found someone at {:?}", next_x_y);
                headcount += 1;
                break;
            }
            i += 1;
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

// #[cfg(test)]
// mod tests {
//     use super::*;

// }
