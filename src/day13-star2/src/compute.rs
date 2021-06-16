use gcd::Gcd;

pub fn next_multiple_after(target: i32, multiple: i32) -> i32 {
    target / multiple * multiple + multiple
}


fn lcm(lhs: u32, rhs: u32) -> u32 {
    lhs * rhs / lhs.gcd(rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_multiple_after() {
        assert_eq!(15, next_multiple_after(12, 5));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(1, 7u32.gcd(13u32));
        assert_eq!(8, 8u32.gcd(64u32));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(91, lcm(7, 13));
        let mut n = 1;

        loop {
            let m = 13 * n;
            if (m - 1) % 7 == 0 {
                println!("{}x7 + 1 = {}x13 = {}", (m - 1) / 7, n, m);
                break
            }

            n += 1;
        }

        assert_eq!(0, 1);
    }

}
