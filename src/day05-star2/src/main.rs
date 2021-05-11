use std::convert::TryInto;
use std::error::Error;

mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data()?;
    let mut seat_ids: Vec<u32> = Vec::with_capacity(lines.len());

    for line in lines {
        seat_ids.push(seat_id(&line));
    }

    seat_ids.sort_unstable();

    let mut last = 0;

    for seat in seat_ids {
        if last != 0 {
            let expected = last + 1;
            if expected != seat {
                println!("your seat is {}", expected);
                break;
            }
        }

        last = seat;
    }

    Ok(())
}

fn range_population(range: (u32, u32)) -> u32 {
    range.1 - range.0 + 1
}

fn half_of(range: (u32, u32)) -> u32 {
    let population = range_population(range);
    let half = population / 2;
    let remainder = population % 2;

    if remainder == 0 {
        half
    } else {
        half + 1
    }
}

fn lower_half(range: (u32, u32)) -> (u32, u32) {
    (range.0, range.0 + half_of(range) - 1)
}

fn upper_half(range: (u32, u32)) -> (u32, u32) {
    (range.1 - half_of(range) + 1, range.1)
}

// probably a good scenario for reduce
fn walk(input: &str, go_up: char, go_down: char) -> u32 {
    let exponent = input.len();
    let num_items = 2_u32.pow(exponent.try_into().unwrap());
    let mut range = (0, num_items - 1);

    for c in input.chars() {
        if c == go_up {
            range = upper_half(range);
        } else if c == go_down {
            range = lower_half(range);
        } else {
            panic!("wtf is {}", c);
        }
    }

    if range.0 != range.1 {
        panic!("the range is wrong: {:?}", range);
    }

    range.0
}

fn find_row(s: &str) -> u32 {
    walk(s, 'B', 'F')
}

fn find_col(s: &str) -> u32 {
    walk(s, 'R', 'L')
}

fn seat_id(s: &str) -> u32 {
    let copy = String::from(s);
    let (row, col) = copy.split_at(7);

    let row_val = find_row(&row);
    let col_val = find_col(&col);

    row_val * 8 + col_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
    }

    #[test]
    fn test_find_row() {
        assert_eq!(find_row("FBFBBFF"), 44);
    }

    #[test]
    fn test_find_col() {
        assert_eq!(find_col("RLR"), 5);
    }

    #[test]
    fn test_half_of() {
        assert_eq!(half_of((0, 1)), 1);
        assert_eq!(half_of((0, 2)), 2);
        assert_eq!(half_of((0, 3)), 2);
        assert_eq!(half_of((0, 4)), 3);
        assert_eq!(half_of((0, 5)), 3);
        assert_eq!(half_of((0, 6)), 4);
    }

    #[test]
    fn test_range_population() {
        assert_eq!(range_population((0, 127)), 128);
        assert_eq!(range_population((0, 2)), 3);
    }

    #[test]
    fn test_lower_half() {
        assert_eq!(lower_half((0, 127)), (0, 63));
        assert_eq!(lower_half((44, 45)), (44, 44));
    }

    #[test]
    fn test_upper_half() {
        assert_eq!(upper_half((0, 7)), (4, 7));
        assert_eq!(upper_half((0, 63)), (32, 63));
        assert_eq!(upper_half((44, 45)), (45, 45));
    }
}
