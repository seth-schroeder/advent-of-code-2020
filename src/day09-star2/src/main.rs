use std::cmp::Ordering;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = lucio::get_input_data(9)?;

    match does_too_much(&lines, 3199139634) {
        Ok(val) => println!("sum of min/max is {}", val),
        Err(_) => panic!("zut alors!"),
    }

    Ok(())
}

fn does_too_much(data: &[u64], target: u64) -> Result<u64, ()> {
    let mut open = 0;
    let mut close = open + 1;

    loop {
        if close >= data.len() {
            break;
        }

        let slice = &data[open..close];
        let sum: u64 = slice.iter().sum();

        match sum.cmp(&target) {
            Ordering::Equal => return Ok(slice.iter().min().unwrap() + slice.iter().max().unwrap()),
            Ordering::Less => close += 1,
            Ordering::Greater => {
                open += 1;
                close = open + 1;
            }
        }
    }

    Err(())
}
