pub mod day;
pub use crate::day::five;
pub use crate::day::four;
pub use crate::day::six;

use fs_err as fs;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::str;

pub fn get_input_data<T: str::FromStr>(day: u8) -> Result<Vec<T>, io::Error>
where
    <T as str::FromStr>::Err: fmt::Debug,
{
    let path = fs::canonicalize(format!("../input-data/day{:02}/input.txt", day))?;
    file_to_lines(path)
}

pub fn get_alternate_input_data<T: str::FromStr>(day: u8, name: &str) -> Result<Vec<T>, io::Error>
where
    <T as str::FromStr>::Err: fmt::Debug,
{
    let path = fs::canonicalize(format!("../../input-data/day{:02}/{}", day, name))?;
    file_to_lines(path)
}

fn file_to_lines<T: str::FromStr>(path: PathBuf) -> Result<Vec<T>, io::Error>
where
    <T as str::FromStr>::Err: fmt::Debug,
{
    let contents = fs::read_to_string(path)?;
    Ok(contents
        .lines()
        .map(String::from)
        .map(|s| s.parse::<T>().unwrap())
        .collect())
}

pub fn chars_match(s: &str, c: char) -> Vec<bool> {
    s.chars().map(|sc| sc == c).collect()
}
