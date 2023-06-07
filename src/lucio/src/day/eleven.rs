use array2d::Array2D;
use std::{eprintln, fmt};

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

// label is used only in the test code sigh
#[allow(dead_code)]
#[derive(Debug)]
struct Angle {
    x: Option<Change>,
    y: Option<Change>,
    label: Direction,
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

fn adjacent_seats2(board: &SeatingArea, x: usize, y: usize) -> Vec<Seat> {
    ANGLES
        .into_iter()
        .filter_map(|angle| {
            let mut iter = SeatIterator {
                seats: board,
                x: Some(x),
                y: Some(y),
                angle: &angle,
            };

            // Skip the current item
            iter.next();

            iter.find(|seat| *seat == Seat::Empty || *seat == Seat::Full)
        })
        .collect()
}

fn adjacent_seats(board: &SeatingArea, x: usize, y: usize) -> Vec<Seat> {
    ANGLES
        .into_iter()
        .filter_map(|angle| {
            let mut iter = SeatIterator {
                seats: board,
                x: Some(x),
                y: Some(y),
                angle: &angle,
            };

            // Skip the current item
            iter.next();

            iter.next()
        })
        .collect()
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

pub fn roll_tape2(seats: &SeatingArea) -> u32 {
    let mut frames = 0;

    // pushing and popping a one element vector is a 1am workaround for juggling ownership and borrowing in a loop
    let mut v = vec![];

    loop {
        frames += 1;

        if v.is_empty() {
            v.push(lights_camera_action2(seats));
            continue;
        }

        let last_area = v.first().unwrap();
        let this_area = lights_camera_action2(last_area);

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

fn lights_camera_action(seats: &SeatingArea) -> SeatingArea {
    let mut next_round = seats.clone();

    for x in 0..seats.num_columns() {
        for y in 0..seats.num_rows() {
            let neighbors = count_neighbors(x, y, seats);
            let index = (y, x);

            if let Some(next_seat) = match seats[index] {
                Seat::Full => {
                    if neighbors > MAX_NEIGHBORS {
                        Some(Seat::Empty)
                    } else {
                        None
                    }
                }
                Seat::Empty => {
                    if neighbors == 0 {
                        Some(Seat::Full)
                    } else {
                        None
                    }
                }
                Seat::Floor => None,
            } {
                next_round[index] = next_seat
            }
        }
    }

    next_round
}

fn lights_camera_action2(seats: &SeatingArea) -> SeatingArea {
    let mut next_round = seats.clone();

    for x in 0..seats.num_columns() {
        for y in 0..seats.num_rows() {
            let neighbors = count_neighbors2(x, y, seats);
            let index = (y, x);

            if let Some(next_seat) = match seats[index] {
                Seat::Full => {
                    if neighbors > 4 {
                        Some(Seat::Empty)
                    } else {
                        None
                    }
                }
                Seat::Empty => {
                    if neighbors == 0 {
                        Some(Seat::Full)
                    } else {
                        None
                    }
                }
                Seat::Floor => None,
            } {
                next_round[index] = next_seat
            }
        }
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

fn count_neighbors(x: usize, y: usize, seats: &SeatingArea) -> usize {
    adjacent_seats(seats, x, y)
        .into_iter()
        .filter(|seat| *seat == Seat::Full)
        .count()
}

fn count_neighbors2(x: usize, y: usize, seats: &SeatingArea) -> usize {
    adjacent_seats2(seats, x, y)
        .into_iter()
        .filter(|seat| *seat == Seat::Full)
        .count()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::get_alternate_input_data;

    use super::*;

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

    fn angle_for(direction: Direction) -> Option<Angle> {
        ANGLES.into_iter().find(|angle| angle.label == direction)
    }

    #[test]
    fn spotcheck() {
        let lines = get_alternate_input_data(11, "starter.txt").unwrap();
        let data = parse_input_data(&lines).unwrap();
        eprintln!("{:?}", adjacent_seats(&data, 0, 0));

        assert_eq!(1, count_neighbors(0, 0, &data));
        assert_eq!(1, count_neighbors(1, 0, &data));
        assert_eq!(0, count_neighbors(2, 0, &data));
        assert_eq!(0, count_neighbors(0, 1, &data));
        assert_eq!(1, count_neighbors(0, 2, &data));
        assert_eq!(0, count_neighbors(0, 3, &data));

        let iter = SeatIterator {
            seats: &data,
            x: Some(0),
            y: Some(0),
            angle: &angle_for(Direction::West).unwrap(),
        };
        assert_eq!(1, iter.count());

        let walk = |direction| {
            SeatIterator {
                seats: &data,
                x: Some(0),
                y: Some(0),
                angle: &angle_for(direction).unwrap(),
            }
            .collect::<Vec<Seat>>()
        };
        assert_eq!(1, walk(Direction::West).len());
        assert_eq!(10, walk(Direction::South).len());
        assert_eq!(11, walk(Direction::East).len());
        assert_eq!(10, walk(Direction::SouthEast).len());
    }

    #[test]
    fn spotcheck2() {
        let lines = get_alternate_input_data(11, "star2-1.txt").unwrap();
        let data = parse_input_data(&lines).unwrap();
        eprintln!("{:?}", adjacent_seats2(&data, 3, 4));

        let lines = get_alternate_input_data(11, "star2-2.txt").unwrap();
        let data = parse_input_data(&lines).unwrap();
        eprintln!("{:?}", adjacent_seats2(&data, 1, 1));

        let lines = get_alternate_input_data(11, "star2-3.txt").unwrap();
        let data = parse_input_data(&lines).unwrap();
        eprintln!("{:?}", adjacent_seats2(&data, 3, 3));

        let lines = get_alternate_input_data(11, "star2-4.txt").unwrap();
        let data = parse_input_data(&lines).unwrap();
        eprintln!("{:?}", adjacent_seats2(&data, 0, 0));
        assert_eq!(0, count_neighbors2(0, 0, &data));

        let next = lights_camera_action2(&data);
        eprintln!("{}\n", textify_board(&next));

        let next = lights_camera_action2(&next);
        eprintln!("{}\n", textify_board(&next));

        let next = lights_camera_action2(&next);
        eprintln!("{}\n", textify_board(&next));

        // I know there's a better way to get text output
        assert!(false);
    }
}
