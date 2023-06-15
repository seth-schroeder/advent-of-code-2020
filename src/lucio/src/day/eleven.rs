pub mod area;
pub mod motion;
pub mod seat;

use area::Area;

pub fn do_the_thing(area: &Area, seats: seat::Seats) -> usize {
    let mut seat_stack = vec![seats];

    loop {
        let last_seats = seat_stack.last().unwrap();
        let these_seats = lights_camera_action(area, last_seats);

        if *last_seats == these_seats {
            break;
        }

        seat_stack.push(these_seats);
    }

    area.count_occupants(&seat_stack.pop().unwrap())
}

fn lights_camera_action(area: &Area, seats: &seat::Seats) -> seat::Seats {
    let mut next_round = seats.clone();

    for x in 0..seats.num_columns() {
        for y in 0..seats.num_rows() {
            let neighbors = area.count_neighbors(seats, x, y);
            let index = (y, x);

            let next_seat = match seats[index] {
                Some(true) => {
                    if neighbors > area.max_neighbors {
                        Some(false)
                    } else {
                        Some(true)
                    }
                }
                Some(false) => {
                    if neighbors == 0 {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
                None => None,
            };

            // dbg!([seats[index], next_seat]);
            next_round[index] = next_seat;
        }
    }

    next_round
}
