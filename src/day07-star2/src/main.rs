use std::error::Error;

mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = lucio::get_input_data(7)?;
    let parsed = parser::hash_parse(&lines);

    println!(
        "well we are carrying around {} bags",
        parser::dive(&parsed, "shiny gold")
    );
    Ok(())
}
