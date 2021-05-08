use std::fs;
use std::io;

// this implementation has got to be pretty bad...
pub fn read_test_data() -> Result<Vec<String>, io::Error> {
    let path = fs::canonicalize("../../input-data/day03-star1/input.txt")?;
    let s = std::fs::read_to_string(path)?;

    let mut mv: Vec<String> = Vec::new();

    for line in s.lines() {
        mv.push(line.trim().to_string());
    }

    Ok(mv)
}

pub fn data_to_lines(data: &Vec<String>) -> Vec<Vec<bool>> {
    let mut v = Vec::with_capacity(data.len());

    for datum in data {
        v.push(process_line(&datum));
    }

    v
}

fn process_line(s: &str) -> Vec<bool> {
    let mut v = Vec::with_capacity(s.len());

    for c in s.chars() {
        if c == '.' {
            v.push(false);
        } else {
            v.push(true);
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_process_line() {
        assert!(Some(false) == process_line(".").pop());
        assert_eq!(vec![true, false, true], process_line("#.#"));
    }

    #[test]
    fn verify_data_to_lines() {
        let data: Vec<String> = vec!["...".to_string(), "##.".to_string()];

        let mut output = data_to_lines(&data);
        assert_eq!(Some(vec![true, true, false]), output.pop());
        assert_eq!(Some(vec![false, false, false]), output.pop());
    }
}
