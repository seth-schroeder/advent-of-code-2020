use std::collections::HashMap;
use std::error::Error;

mod test_data;

fn valid_hash(h: &HashMap<String, String>) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for k in required_keys {
        if !h.contains_key(k) {
            return false;
        }
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let hashes = test_data::data_to_hashes(&test_data::read_test_data()?);
    let mut valid = 0;

    for h in hashes {
        if valid_hash(&h) {
            valid += 1;
        }
    }

    println!("Yup I found {} valid items", valid);

    Ok(())
}
