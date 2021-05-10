fn main() {
    println!("Hello, world!");
}

fn range_population(floor: u32, ceiling: u32) -> u32 {
    ceiling - floor + 1
}

fn half_of(floor: u32, ceiling: u32) -> u32 {
    let population = range_population(floor, ceiling);
    let half = population / 2;
    let remainder = population % 2;

    if remainder == 0 {
        half
    } else {
        half + 1
    }
}

fn lower_half(floor: u32, ceiling: u32) -> (u32, u32) {
    (floor, floor + half_of(floor, ceiling) - 1)
}

fn upper_half(floor: u32, ceiling: u32) -> (u32, u32) {
    (ceiling - half_of(floor, ceiling) + 1, ceiling)
}


// probably a good scenario for reduce
fn walk(s: &str) -> u32 {
    let mut range = (0, 127);

    for c in s.chars() {
        if c == 'B' {
            range = upper_half(range.0, range.1);
        } else if c == 'F' {
            range = lower_half(range.0, range.1);
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
    fn test_walk() {
        assert_eq!(walk("FBFBBFF"), 44);
    }

    #[test]
    fn test_half_of() {
        assert_eq!(half_of(0, 1), 1);
        assert_eq!(half_of(0, 2), 2);
        assert_eq!(half_of(0, 3), 2);
        assert_eq!(half_of(0, 4), 3);
        assert_eq!(half_of(0, 5), 3);
        assert_eq!(half_of(0, 6), 4);
    }

    #[test]
    fn test_range_population() {
        assert_eq!(range_population(0, 127), 128);
        assert_eq!(range_population(0, 2), 3);
    }

    #[test]
    fn test_lower_half() {
        assert_eq!(lower_half(0, 127), (0, 63));
        assert_eq!(lower_half(44, 45), (44, 44));
    }

    #[test]
    fn test_upper_half() {
        assert_eq!(upper_half(0, 7), (4, 7));
        assert_eq!(upper_half(0, 63), (32, 63));
        assert_eq!(upper_half(44, 45), (45, 45));
    }
}
