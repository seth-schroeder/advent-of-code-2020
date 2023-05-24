use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input:Vec<String> = lucio::get_input_data(2)?;
    let mut valid = 0;
    let mut invalid = 0;

    for line in input {
        let pieces: Vec<&str> = line.split(" ").collect();

        let range = pieces[0];
        let range_pieces: Vec<&str> = range.split("-").collect();
        let open: usize = range_pieces[0].parse().unwrap();
        let close: usize = range_pieces[1].parse().unwrap();

        let pat = pieces[1].replace(":", "");

        let password = pieces[2];

        let hits: Vec<&str> = password.matches(&pat).collect();
        if hits.len() >= open && hits.len() <= close {
            valid += 1;
        } else {
            invalid += 1;
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
