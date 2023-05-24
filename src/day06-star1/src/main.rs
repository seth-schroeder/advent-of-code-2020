use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = lucio::get_input_data(6)?;
    let groups = lucio::day::six::lines_to_groups(&lines);

    let sum = groups.iter().fold(0, |memo, group| memo + group.len());

    println!("The total # of questions answered was {}", sum);

    Ok(())
}
