use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let numbers: Vec<u32> = lucio::get_input_data(1)?;

    // right in the n^2
    for outer in &numbers {
        for inner in &numbers {
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
