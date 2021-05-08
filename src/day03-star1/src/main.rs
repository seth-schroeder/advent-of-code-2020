use std::error::Error;

mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let rows = test_data::data_to_lines(&test_data::read_test_data()?);
    let mut col = 0;
    let mut trees = 0;

    for row in &rows {
        if let Some(thing) = row.get(col) {
            if *thing {
                trees += 1;
            }
        }
        col = (col + 3) % row.len();
    }

    println!("yo we would have hit {} trees", trees);

    Ok(())
}
