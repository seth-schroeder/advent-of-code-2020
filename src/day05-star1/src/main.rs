use lucio::day;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = lucio::get_input_data(5)?;
    let mut scores = HashMap::new();

    for line in lines {
        let score = day::five::seat_id(&line);
        scores.insert(score, line);
    }

    let largest = scores.keys().max().unwrap();
    println!(
        "The largest score was {} for {}",
        largest,
        scores.get(largest).unwrap()
    );

    Ok(())
}
