pub type Seat = Option<bool>;
pub type Seats = array2d::Array2D<Seat>;
pub type Adjacency = fn(iter: SeatIterator) -> Seat;
use super::motion;

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// this seems awkward, but std::fmt doesn't do what I want with Option
pub fn format(seat: &Option<bool>) -> String {
    match seat {
        Some(true) => String::from("#"),
        Some(false) => String::from("L"),
        None => String::from("."),
    }
}

pub struct SeatIterator<'a> {
    pub seats: &'a Seats,
    pub x: Option<usize>,
    pub y: Option<usize>,
    pub slope: &'a motion::Slope,
}

impl Iterator for SeatIterator<'_> {
    type Item = Seat;

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(x), Some(y)) = (self.x, self.y) {
            self.x = motion::move_in_direction(x, self.slope.x);
            self.y = motion::move_in_direction(y, self.slope.y);
            self.seats.get(y, x).copied()
        } else {
            None
        }
    }
}

pub fn parse_input_data(lines: &[String]) -> Result<Seats, array2d::Error> {
    let seats = lines
        .iter()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Some(true),
                    'L' => Some(false),
                    '.' => None,
                    _ => panic!("bad data!"),
                })
                .collect::<Vec<Seat>>()
        })
        .collect::<Vec<Vec<Seat>>>();

    array2d::Array2D::from_rows(&seats)
}
