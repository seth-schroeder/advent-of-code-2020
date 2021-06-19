mod test_data;
use rand::seq::SliceRandom;
use std::error::Error;

const SUM: i32 = 2020;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data()?;
    let mut loops = 0;

    let mut rng = rand::thread_rng();
    let mut outer_lines = lines.clone();
    let mut middle_lines = lines.clone();
    let mut inner_lines = lines.clone();

    outer_lines.shuffle(&mut rng);
    middle_lines.shuffle(&mut rng);
    inner_lines.shuffle(&mut rng);

    'outer: for outer_value in outer_lines {
        for middle_value in &middle_lines {
            if outer_value + middle_value > SUM {
                continue;
            }
            for inner_value in &inner_lines {
                loops += 1;
                if inner_value + middle_value + outer_value == SUM {
                    println!(
                        "inner: {}, middle: {}, outer: {}, product: {}",
                        inner_value,
                        middle_value,
                        outer_value,
                        inner_value * middle_value * outer_value
                    );
                    break 'outer;
                }
            }
        }
    }

    println!("{} loops", loops);

    Ok(())
}
