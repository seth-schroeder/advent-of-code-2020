fn main() {
    println!("Hello, world!");
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
fn find_row(s: &str) -> u32 {
    let mut range = (0, 127);

    for c in s.chars() {
        if c == 'B' {
            range = upper_half(range);
        } else if c == 'F' {
            range = lower_half(range);
        }
    }

    if range.0 != range.1 {
        panic!("the range is wrong: {:?}", range);
    }

    range.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_row() {
        assert_eq!(find_row("FBFBBFF"), 44);
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
