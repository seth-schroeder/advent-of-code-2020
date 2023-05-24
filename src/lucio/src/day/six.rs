use std::collections::HashSet;

pub fn lines_to_groups(v: &[String]) -> Vec<HashSet<char>> {
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
