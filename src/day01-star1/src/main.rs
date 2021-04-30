mod test_data;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data()?;

    for outer in &lines {
        for inner in &lines {
            if 2020 == inner + outer {
                println!(
                    "inner: {}, outer: {}, product: {}",
                    inner,
                    outer,
                    inner * outer
                );
                break;
            }
        }
    }
    Ok(())
}
