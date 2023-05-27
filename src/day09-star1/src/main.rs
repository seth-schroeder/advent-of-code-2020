use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = lucio::get_input_data(9)?;

    match does_too_much(&lines) {
        Ok(val) => println!("the problem was {}", val),
        Err(_) => println!("no problem, ironically"),
    }

    Ok(())
}

fn does_too_much(data: &[u64]) -> Result<u64, ()> {
    let window_size = 25;
    let mut open = 0;
    let mut close = window_size;

    loop {
        if close >= data.len() {
            break;
        }

        let candidate = data[close];

        let mut matched = false;

        // this hurts just looking at it
        'outer: for outer in data[open..close].iter() {
            for inner in data[open..close].iter() {
                if outer == inner {
                    continue;
                } else if candidate == outer + inner {
                    matched = true;
                    break 'outer;
                }
            }
        }

        if !matched {
            return Ok(candidate);
        }

        open += 1;
        close += 1;
    }

    Err(())
}
