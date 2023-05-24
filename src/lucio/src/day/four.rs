use std::collections::HashMap;

pub fn data_to_hashes(data: &[String]) -> Vec<HashMap<String, String>> {
    let mut v = Vec::with_capacity(data.len());
    let mut h = HashMap::new();

    for datum in data {
        if datum.is_empty() {
            v.push(h.clone());
            h.clear();
        }

        for pair in datum.split_whitespace() {
            let mut pieces: Vec<&str> = pair.split(':').collect();
            if let Some(value) = pieces.pop() {
                if let Some(key) = pieces.pop() {
                    h.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    // did the file end in a record?
    if !h.is_empty() {
        v.push(h);
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_data_to_lines() {
        let data: Vec<String> = vec!["foo:bar".to_string()];

        let mut output = data_to_hashes(&data);
        assert_eq!(1, output.len());

        let h = output.pop().unwrap();
        assert_eq!(Some(&"bar".to_string()), h.get("foo"));
    }
}
