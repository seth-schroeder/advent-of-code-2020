use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = lucio::get_input_data(3)?;
    let rows: Vec<Vec<bool>> = lines.iter().map(|s| lucio::chars_match(s, '#')).collect();
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
