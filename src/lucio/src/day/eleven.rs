use array2d::Array2D;
use std::fmt;

const MAX_NEIGHBORS: usize = 3;

#[derive(Debug)]
enum Change {
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug)]
struct Angle {
    x: Option<Change>,
    y: Option<Change>,
    label: Direction,
}

fn angle_for(direction: Direction) -> Option<Angle> {
    ANGLES.into_iter().find(|angle| angle.label == direction)
}

const ANGLES: [Angle; 8] = [
    Angle {
        label: Direction::North,
        x: None,
        y: Some(Change::Decrement),
    },
    Angle {
        label: Direction::NorthEast,
        x: Some(Change::Increment),
        y: Some(Change::Decrement),
    },
    Angle {
        label: Direction::East,
        x: Some(Change::Increment),
        y: None,
    },
    Angle {
        label: Direction::SouthEast,
        x: Some(Change::Increment),
        y: Some(Change::Increment),
    },
    Angle {
        label: Direction::South,
        x: None,
        y: Some(Change::Increment),
    },
    Angle {
        label: Direction::SouthWest,
        x: Some(Change::Decrement),
        y: Some(Change::Increment),
    },
    Angle {
        label: Direction::West,
        x: Some(Change::Decrement),
        y: None,
    },
    Angle {
        label: Direction::NorthWest,
        x: Some(Change::Decrement),
        y: Some(Change::Decrement),
    },
];

#[derive(Debug)]
struct SeatIterator<'a> {
    seats: &'a SeatingArea,
    x: Option<usize>,
    y: Option<usize>,
    angle: &'a Angle,
}

impl Iterator for SeatIterator<'_> {
    type Item = Seat;

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(x), Some(y)) = (self.x, self.y) {
            self.x = move_in_direction(x, &self.angle.x);
            self.y = move_in_direction(y, &self.angle.y);
            self.seats.get(y, x).copied()
        } else {
            None
        }
    }
}

fn move_in_direction(index: usize, potential_change: &Option<Change>) -> Option<usize> {
    match potential_change {
        None => Some(index),
        Some(change) => match change {
            Change::Increment => index.checked_add(1),
            Change::Decrement => index.checked_sub(1),
        },
    }
}

fn adjacent_seats(board: &SeatingArea, x: usize, y: usize) -> Vec<Seat> {
    // TODO `fold` and `Vec` have been difficult
    ANGLES
        .into_iter()
        .filter_map(|angle| {
            let mut iter = SeatIterator {
                seats: board,
                x: Some(x),
                y: Some(y),
                angle: &angle,
            };

            iter.next();
            let seat = iter.next();
            eprintln!(
                "x:{:?}, y:{:?}, a:{:?}, s:{:?}",
                iter.x, iter.y, angle.label, seat
            );
            seat
        })
        .collect()
}

fn textify_board(board: &SeatingArea) -> String {
    board
        .rows_iter()
        .map(|row_iter| {
            row_iter
                .map(|seat| format!("{}", seat))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

// hmm the Hollywood metaphors are wan tonight
pub fn roll_tape(seats: &SeatingArea) -> u32 {
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
        let this_area = lights_camera_action(last_area);

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

pub fn parse_input_data(lines: &[String]) -> Result<Array2D<Seat>, array2d::Error> {
    let seats = lines
        .iter()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Seat::Full,
                    'L' => Seat::Empty,
                    '.' => Seat::Floor,
                    _ => panic!("bad data!"),
                })
                .collect::<Vec<Seat>>()
        })
        .collect::<Vec<Vec<Seat>>>();

    Array2D::from_rows(&seats)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seat {
    Empty,
    Full,
    Floor,
}

pub type SeatingArea = Array2D<Seat>;

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

        // hello there, doubleplus extra time in Purgatory
        for x_ in x_min..x_max {
            for y_ in y_min..y_max {
                if x_y == (x_, y_) {
                    continue;
                }

                v.push((x_, y_));
            }
        }

        v
    }
}

fn lights_camera_action(seats: &SeatingArea) -> SeatingArea {
    let mut next_round = seats.clone();
    let mut x_y = (0, 0);

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

fn count_neighbors(x_y: (usize, usize), seats: &SeatingArea) -> usize {
    // let mut headcount = 0;
    // let to_visit = Seat::adjacent_indices(x_y, seats);

    // for nx_ny in to_visit {
    //     if let Seat::Full = seats[nx_ny] {
    //         headcount += 1;
    //     }
    // }

    // headcount
    let (x, y) = x_y;
    adjacent_seats(seats, x, y)
        .into_iter()
        .filter(|seat| *seat == Seat::Full)
        .count()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::get_alternate_input_data;

    use super::*;

    #[test]
    fn spotcheck() {
        let lines = get_alternate_input_data(11, "starter.txt").unwrap();
        let data = parse_input_data(&lines).unwrap();
        eprintln!("{:?}", adjacent_seats(&data, 0, 0));

        assert_eq!(1, count_neighbors((0, 0), &data));
        assert_eq!(1, count_neighbors((1, 0), &data));
        assert_eq!(0, count_neighbors((2, 0), &data));
        assert_eq!(0, count_neighbors((0, 1), &data));
        assert_eq!(1, count_neighbors((0, 2), &data));
        assert_eq!(0, count_neighbors((0, 3), &data));

        let iter = SeatIterator {
            seats: &data,
            x: Some(0),
            y: Some(0),
            angle: &angle_for(Direction::West).unwrap(),
        };
        assert_eq!(1, iter.count());

        let make_iter = |direction| {
            SeatIterator {
                seats: &data,
                x: Some(0),
                y: Some(0),
                angle: &angle_for(direction).unwrap(),
            }.collect::<Vec<Seat>>()
        };
        assert_eq!(1, make_iter(Direction::West).len());
        assert_eq!(10, make_iter(Direction::East).len());
        assert_eq!(10, make_iter(Direction::SouthEast).len());
    }
}
