use lucio::day;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = lucio::get_input_data(5)?;
    let mut seat_ids: Vec<u32> = Vec::with_capacity(lines.len());

    for line in lines {
        seat_ids.push(day::five::seat_id(&line));
    }

    seat_ids.sort_unstable();

    let mut last = 0;

    for seat in seat_ids {
        if last != 0 {
            let expected = last + 1;
            if expected != seat {
                println!("your seat is {}", expected);
                break;
            }
        }

        last = seat;
    }

    Ok(())
}
