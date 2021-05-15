use std::error::Error;

mod parser;
mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data("day07-star2/small.txt")?;
    let results = parser::parse(&lines);

    Ok(())
}
