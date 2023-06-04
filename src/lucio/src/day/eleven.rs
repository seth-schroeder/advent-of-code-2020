use array2d::Array2D;
use std::fmt;

const MAX_NEIGHBORS: u8 = 3;

enum Motion {
    GoForward,
    GoBackward,
}

struct SeatIterator<'a> {
    seats: &'a SeatingArea,
    row_index: Option<usize>,
    row_delta: Option<Motion>,
    column_index: Option<usize>,
    column_delta: Option<Motion>,
}

impl Iterator for SeatIterator<'_> {
    type Item = Seat;

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(row_index), Some(column_index)) = (self.row_index, self.column_index) {
            self.row_index = move_in_direction(row_index, &self.row_delta);
            self.column_index = move_in_direction(column_index, &self.column_delta);
            self.seats.get(row_index, column_index).copied()
        } else {
            None
        }
    }
}

fn move_in_direction(index: usize, motion: &Option<Motion>) -> Option<usize> {
    match motion {
        None => Some(index),
        Some(motion_) => match motion_ {
            Motion::GoForward => index.checked_add(1),
            Motion::GoBackward => index.checked_sub(1),
        },
    }
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

pub fn parse_input_data() -> Result<Array2D<Seat>, array2d::Error> {
    let lines: Vec<String> = crate::get_input_data(11).unwrap();
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

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn check_due_east() {
        let seats = Array2D::filled_with(Seat::Empty, 2, 2);
        let mut iter = SeatIterator {
            seats: &seats,
            row_index: Some(0),
            row_delta: Some(Motion::GoForward),
            column_index: Some(0),
            column_delta: Some(Motion::GoForward),
        };
        assert_eq!(Some(Seat::Empty), iter.next());
        assert_eq!(Some(1), iter.row_index);
        assert_eq!(Some(1), iter.column_index);
        assert_eq!(Some(Seat::Empty), iter.next());
        assert_eq!(Some(2), iter.row_index);
        assert_eq!(Some(2), iter.column_index);
        assert_eq!(None, iter.next())
    }
}
