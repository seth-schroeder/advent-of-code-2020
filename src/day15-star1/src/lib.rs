mod compute;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    dbg!(compute::nth_number_spoken(2019, &vec![1, 3, 2]));
    Ok(())
}
