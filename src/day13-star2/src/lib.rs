use std::error::Error;
use std::fs;
use std::io;

mod compute;

pub fn run() -> Result<(), Box<dyn Error>> {
    // let (earliest, durations) =
    //     parse_test_data(read_test_data("day13-star1/actual.txt")?).unwrap();

    // let h = compute::delta_hash(earliest, &durations);
    // let k = h.keys().min().unwrap();
    // let v = h.get(k).unwrap();

    // println!("(pulls {:?} out of a hat)", k * v);

    Ok(())
}

fn parse_test_data(data: Vec<String>) -> Option<(i32, Vec<i32>)> {
    let earliest = data.first().unwrap().parse().unwrap();
    let raw_durations: Vec<&str> = data.last().unwrap().split(',').collect();

    Some((
        earliest,
        raw_durations
            .iter()
            .filter(|c| *c != &"x")
            .map(|s| s.parse().unwrap())
            .collect(),
    ))
}

fn read_test_data(relative_file_name: &str) -> Result<Vec<String>, io::Error> {
    let path = fs::canonicalize(format!("../../input-data/{}", relative_file_name))?;
    let s = fs::read_to_string(path)?;
    let mut v = Vec::new();
    for line in s.lines() {
        v.push(line.to_string());
    }
    Ok(v)
}
