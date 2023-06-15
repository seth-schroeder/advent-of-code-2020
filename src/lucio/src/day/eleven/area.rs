use super::motion;
use super::seat;

pub enum ScopeToScan {
    // the compass rose surrounding the point
    Adjacent,

    // the compass rose to the edge of the board
    NextVisible,
}

pub struct Area {
    pub scope: ScopeToScan,
    pub max_neighbors: usize,
}

impl Area {
    pub fn count_neighbors(&self, seats: &seat::Seats, x: usize, y: usize) -> usize {
        self.adjacent(seats, x, y)
            .into_iter()
            // .filter(|seat| if let Some(true) = *seat { true } else { false })
            .filter(|seat| matches!(*seat, Some(true)))
            .count()
    }

    pub fn adjacent(&self, seats: &seat::Seats, x: usize, y: usize) -> Vec<seat::Seat> {
        let scope = &self.scope;

        motion::SLOPES
            .iter()
            .filter_map(|slope| {
                let mut iter = seat::SeatIterator {
                    seats,
                    x: Some(x),
                    y: Some(y),
                    slope,
                };

                // discard the current spot, we need to look around it
                iter.next();

                match scope {
                    ScopeToScan::Adjacent => iter.next(),
                    ScopeToScan::NextVisible => iter.find(Option::is_some),
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn count_occupants(&self, seats: &seat::Seats) -> usize {
        seats.elements_row_major_iter().fold(
            0,
            |acc, seat| {
                if let Some(true) = seat {
                    acc + 1
                } else {
                    acc
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::day::eleven::*;

    // this is way too handy and non-trivial
    #[allow(dead_code)]
    fn textify_board(seats: &seat::Seats) -> String {
        seats
            .rows_iter()
            .map(|row_iter| row_iter.map(seat::format).collect::<Vec<String>>().join(""))
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn spotcheck() {
        let area = area::Area {
            max_neighbors: 4,
            scope: area::ScopeToScan::NextVisible,
        };

        let lines = crate::get_alternate_input_data(11, "star2-4.txt").unwrap();
        let seats = seat::parse_input_data(&lines).unwrap();
        assert_eq!(0, area.count_occupants(&seats));

        let seats = lights_camera_action(&area, &seats);

        // this comes from test data tied to my AoC account (which I .gitignore)
        let seats = lights_camera_action(&area, &seats);
        assert_eq!(7, area.count_occupants(&seats));
    }
}
