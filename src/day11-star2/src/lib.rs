use lucio::day::eleven::{Seat, SeatingArea};
use std::convert::TryFrom;

pub fn run() {
    let input_data = lucio::day::eleven::parse_input_data().unwrap();
    let mut last_round = input_data;
    let mut frames = 0;

    loop {
        let next_round = evolve(&last_round);
        frames += 1;

        if next_round == last_round {
            break;
        }

        last_round = next_round;
    }

    println!(
        "{} neighbors, it took {} frames, finished with \n{}",
        count_neighbors(&last_round),
        frames,
        textify_board(&last_round)
    );
}

fn count_neighbors(board: &SeatingArea) -> usize {
    board
        .as_column_major()
        .iter()
        .filter(|seat| match seat {
            Seat::Full => true,
            _ => false,
        })
        .count()
}

fn scan_for_neighbors(board: &SeatingArea, x: usize, y: usize) -> u32 {
    let compass_rose: Vec<(i8, i8)> = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let ix = i8::try_from(x).unwrap();
    let iy = i8::try_from(y).unwrap();
    let mut neighbors = 0;

    for (y_slope, x_slope) in compass_rose {
        let mut at_y = iy + y_slope;
        let mut at_x = ix + x_slope;

        loop {
            if at_y < 0 || at_x < 0 {
                break;
            }

            match board.get(&usize::try_from(at_y).unwrap()) {
                None => break,
                Some(xh) => match xh.get(&usize::try_from(at_x).unwrap()) {
                    None => break,
                    Some(seat) => match seat {
                        Seat::Floor => (),
                        Seat::Full => {
                            neighbors += 1;
                            break;
                        }
                        Seat::Empty => break,
                    },
                },
            }

            at_y += y_slope;
            at_x += x_slope;
        }
    }

    neighbors
}

fn evolve(board: &SeatingArea) -> SeatingArea {
    let mut b = board.clone();

    for (y_index, y_hash) in board {
        let by_hash = b.get_mut(y_index).unwrap();

        for (x_index, seat) in y_hash {
            let new_seat = match seat {
                Seat::Floor => Seat::Floor,
                Seat::Empty => {
                    let neighbors = scan_for_neighbors(board, *x_index, *y_index);
                    if neighbors == 0 {
                        Seat::Full
                    } else {
                        Seat::Empty
                    }
                }
                Seat::Full => {
                    let neighbors = scan_for_neighbors(board, *x_index, *y_index);
                    if neighbors >= 5 {
                        Seat::Empty
                    } else {
                        Seat::Full
                    }
                }
            };

            by_hash.insert(*x_index, new_seat);
        }
    }

    b
}

fn textify_board(board: &SeatingArea) -> String {
    let mut s = Vec::new();

    for yh in board.values() {
        let mut v = Vec::new();
        for seat in yh.values() {
            v.push(format!("{}", seat))
        }
        s.push(v.join(""));
    }

    s.join("\n")
}

// type SeatingAreaIndex = usize;
// type SeatingArea = BTreeMap<SeatingAreaIndex, BTreeMap<SeatingAreaIndex, Seat>>;

// fn parse_test_data() -> Result<SeatingArea, io::Error> {
//     let lines: Vec<String> = lucio::get_input_data(11)?;

//     let mut yh = SeatingArea::new();
//     let mut y = 0;

//     for line in lines {
//         let mut xh = BTreeMap::new();
//         let mut x = 0;

//         for c in line.trim().chars() {
//             xh.insert(
//                 x,
//                 match c {
//                     '#' => Seat::Full,
//                     'L' => Seat::Empty,
//                     '.' => Seat::Floor,
//                     _ => panic!("bad data!"),
//                 },
//             );
//             x += 1;
//         }

//         yh.insert(y, xh);
//         y += 1;
//     }

//     Ok(yh)
// }
