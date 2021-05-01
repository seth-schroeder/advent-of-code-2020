mod test_data;
use std::error::Error;

const SUM: i32 = 2020;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data()?;
    let mut loops = 0;

    'outer: for outer_value in &lines {
        for middle_value in &lines {
            if outer_value + middle_value > SUM {
                continue;
            }
            for inner_value in &lines {
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
