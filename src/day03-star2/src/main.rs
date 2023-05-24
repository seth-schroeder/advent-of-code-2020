use std::error::Error;

fn smack_into_trees(each_x: usize, each_y: usize) -> u64 {
    let lines: Vec<String> = lucio::get_input_data(3).expect("something to parse?");
    let rows: Vec<Vec<bool>> = lines.iter().map(|s| lucio::chars_match(s, '#')).collect();
    let mut col = 0;
    let mut trees = 0;
    let mut cur_row = 0;

    for row in &rows {
        if cur_row % each_y == 0 {
            if let Some(thing) = row.get(col) {
                if *thing {
                    trees += 1;
                }
            }
            col = (col + each_x) % row.len();
        }
        cur_row += 1;
    }

    println!(
        "yo we would have hit {} trees for {}:{}",
        trees, each_x, each_y
    );

    trees
}

fn main() -> Result<(), Box<dyn Error>> {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product: u64 = 1;

    for slope in &slopes {
        let x = slope.0;
        let y = slope.1;

        product *= smack_into_trees(x, y);
    }

    println!("ze product is {}", product);

    Ok(())
}
