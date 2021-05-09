use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let hashes = test_data::data_to_hashes(&test_data::read_test_data()?);
    let mut valid = 0;

    for h in hashes {
        if valid_keys(&h) && valid_values(&h) {
            valid += 1;
        }
    }

    println!("Yup I found {} valid items", valid);

    Ok(())
}

fn is_date_in_range(s: &str, floor: u32, ceiling: u32) -> bool {
    match s.parse::<u32>() {
        Ok(year) => year >= floor && year <= ceiling,
        Err(_) => false,
    }
}

fn is_valid_byr(s: &str) -> bool {
    is_date_in_range(s, 1920, 2002)
}

fn is_valid_iyr(s: &str) -> bool {
    is_date_in_range(s, 2010, 2020)
}

fn is_valid_eyr(s: &str) -> bool {
    is_date_in_range(s, 2020, 2030)
}

fn is_hgt_in_range(s: &str, suffix: &str, floor: u32, ceiling: u32) -> bool {
    let pat = format!(r"^(\d+){}$", suffix.to_string());
    let r = Regex::new(&pat).unwrap();

    match r.captures(s) {
        Some(captures) => match captures[1].parse::<u32>() {
            Ok(i) => i >= floor && i <= ceiling,
            Err(_) => false,
        },
        None => false,
    }
}

fn is_valid_hgt(s: &str) -> bool {
    is_hgt_in_range(s, "cm", 150, 193) || is_hgt_in_range(s, "in", 59, 76)
}

fn is_valid_hcl(s: &str) -> bool {
    let r = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    r.is_match(s)
}

fn is_valid_ecl(s: &str) -> bool {
    let r = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    r.is_match(s)
}

fn is_valid_pid(s: &str) -> bool {
    let r = Regex::new(r"^[0-9]{9}$").unwrap();
    r.is_match(s)
}

fn valid_keys(h: &HashMap<String, String>) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for k in required_keys {
        if !h.contains_key(k) {
            return false;
        }
    }

    true
}

fn valid_values(h: &HashMap<String, String>) -> bool {
    is_valid_byr(h.get("byr").unwrap())
        && is_valid_ecl(h.get("ecl").unwrap())
        && is_valid_eyr(h.get("eyr").unwrap())
        && is_valid_hcl(h.get("hcl").unwrap())
        && is_valid_hgt(h.get("hgt").unwrap())
        && is_valid_iyr(h.get("iyr").unwrap())
        && is_valid_pid(h.get("pid").unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_byr() {
        assert!(!is_valid_byr("1919"));
        assert!(is_valid_byr("1920"));
        assert!(is_valid_byr("2002"));
        assert!(!is_valid_byr("2003"));
    }

    #[test]
    fn verify_iyr() {
        assert!(!is_valid_iyr("2009"));
        assert!(is_valid_iyr("2010"));
        assert!(is_valid_iyr("2020"));
        assert!(!is_valid_iyr("2021"));
    }

    #[test]
    fn verify_eyr() {
        assert!(!is_valid_eyr("2019"));
        assert!(is_valid_eyr("2020"));
        assert!(is_valid_eyr("2030"));
        assert!(!is_valid_eyr("2031"));
    }

    #[test]
    fn verify_hgt() {
        assert!(!is_valid_hgt("garbage"));

        assert!(!is_valid_hgt("58in"));
        assert!(is_valid_hgt("59in"));
        assert!(is_valid_hgt("76in"));
        assert!(!is_valid_hgt("77in"));

        assert!(!is_valid_hgt("149cm"));
        assert!(is_valid_hgt("150cm"));
        assert!(is_valid_hgt("193cm"));
        assert!(!is_valid_hgt("194cm"));
    }

    #[test]
    fn verify_hcl() {
        assert!(!is_valid_hcl("garbage"));
        assert!(is_valid_hcl("#123abc"));
        assert!(!is_valid_hcl("#123abz"));
    }

    #[test]
    fn verify_ecl() {
        assert!(!is_valid_ecl("gndn"));
        assert!(is_valid_ecl("hzl"));
    }

    #[test]
    fn verify_pid() {
        assert!(!is_valid_pid("gndn"));
        assert!(is_valid_pid("012345678"));
    }
}
