use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io;

pub fn run() {
    let input_data = read_test_data("day11-star1/actual.txt").unwrap();
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

fn count_neighbors(board: &Board) -> u32 {
    let mut headcount = 0;

    for xh in board.values() {
        for seat in xh.values() {
            if let Seat::Occupied = seat {
                headcount += 1
            }
        }
    }

    headcount
}

fn scan_for_neighbors(board: &Board, x: usize, y: usize) -> u32 {
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

    for slope in compass_rose {
        let (y_slope, x_slope) = slope;
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
                    Some(seat) => {
                        match seat {
                            Seat::Floor => (),
                            Seat::Occupied => {
                                neighbors += 1;
                                break;
                            }
                            Seat::Empty => break,
                        }
                    }
                },
            }

            at_y += y_slope;
            at_x += x_slope;
        }
    }

    neighbors
}

fn evolve(board: &Board) -> Board {
    let mut b = board.clone();

    for (y_index, y_hash) in board {
        let by_hash = b.get_mut(y_index).unwrap();

        for (x_index, seat) in y_hash {
            let new_seat = match seat {
                Seat::Floor => Seat::Floor,
                Seat::Empty => {
                    let neighbors = scan_for_neighbors(board, *x_index, *y_index);
                    if neighbors == 0 {
                        Seat::Occupied
                    } else {
                        Seat::Empty
                    }
                }
                Seat::Occupied => {
                    let neighbors = scan_for_neighbors(board, *x_index, *y_index);
                    if neighbors >= 5 {
                        Seat::Empty
                    } else {
                        Seat::Occupied
                    }
                }
            };

            by_hash.insert(*x_index, new_seat);
        }
    }

    b
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::Occupied => '#',
                Seat::Empty => 'L',
                Seat::Floor => '.',
            }
        )
    }
}

fn textify_board(board: &Board) -> String {
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

type BoardIndex = usize;
type Board = BTreeMap<BoardIndex, BTreeMap<BoardIndex, Seat>>;

fn read_test_data(relative_file_name: &str) -> Result<Board, io::Error> {
    let path = fs::canonicalize(format!("../../input-data/{}", relative_file_name))?;
    let s = fs::read_to_string(path)?;

    let mut yh = Board::new();
    let mut y = 0;

    for line in s.lines() {
        let mut xh = BTreeMap::new();
        let mut x = 0;

        for c in line.trim().chars() {
            xh.insert(
                x,
                match c {
                    '#' => Seat::Occupied,
                    'L' => Seat::Empty,
                    '.' => Seat::Floor,
                    _ => panic!("bad data!"),
                },
            );
            x += 1;
        }

        yh.insert(y, xh);
        y += 1;
    }

    Ok(yh)
}

