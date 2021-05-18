use std::error::Error;

mod parser;
mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data("day07-star1/haiku.txt")?;
    let parsed = parser::hash_parse(&lines);

    println!(
        "well we are carrying around {} bags",
        parser::dive(&parsed, "shiny gold")
    );
    Ok(())
}
