use std::collections::HashSet;
use std::error::Error;

mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data()?;
    let groups = lines_to_groups(&lines);

    let sum = groups.iter().fold(0, |memo, group| memo + group.len());

    println!("The total # of questions answered was {}", sum);

    Ok(())
}

fn lines_to_groups(v: &[String]) -> Vec<HashSet<char>> {
    let mut output = Vec::new();
    let mut bucket: HashSet<char> = HashSet::new();

    for line in v {
        if line.is_empty() {
            output.push(bucket.clone());
            bucket.clear();
        } else {
            for c in line.chars() {
                bucket.insert(c);
            }
        }
    }

    if !bucket.is_empty() {
        output.push(bucket);
    }

    output
}
