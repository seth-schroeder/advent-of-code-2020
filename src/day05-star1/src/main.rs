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

fn bottom_half(floor: u32, ceiling: u32) -> (u32, u32) {
    (floor, floor + half_of(floor, ceiling) - 1)
}

fn top_half(floor: u32, ceiling: u32) -> (u32, u32) {
    (ceiling - half_of(floor, ceiling) + 1, ceiling)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_bottom_half() {
        assert_eq!(bottom_half(0, 127), (0, 63));
    }

    #[test]
    fn test_top_half() {
        assert_eq!(top_half(0, 7), (4, 7));
        assert_eq!(top_half(0, 63), (32, 63));
    }
}
