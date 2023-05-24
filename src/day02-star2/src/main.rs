use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = lucio::get_input_data(2)?;
    let mut valid = 0;
    let mut invalid = 0;

    for line in input {
        let pieces: Vec<&str> = line.split(' ').collect();

        let range = pieces[0];
        let range_pieces: Vec<&str> = range.split('-').collect();
        let open: usize = range_pieces[0].parse().unwrap();
        let close: usize = range_pieces[1].parse().unwrap();

        let password = pieces[2];
        let mut occurs = HashMap::new();

        for (i, c) in password.char_indices() {
            let mut s = String::new();
            s.push(c);
            occurs.insert(i + 1, s);
        }

        let pat = pieces[1].replace(':', "");

        let first_match = match occurs.get(&open) {
            Some(s) => *s == pat,
            None => false,
        };

        let last_match = match occurs.get(&close) {
            Some(s) => *s == pat,
            None => false,
        };

        if (first_match && last_match) || !(first_match || last_match) {
            invalid += 1;
        } else {
            valid += 1
        }
    }

    println!(
        "total: {}, invalid: {}, valid: {}",
        invalid + valid,
        invalid,
        valid
    );

    Ok(())
}
