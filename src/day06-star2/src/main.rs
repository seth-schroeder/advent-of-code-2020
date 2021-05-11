use std::collections::HashMap;
use std::error::Error;

mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data()?;
    let (v_members, v_answers) = lines_to_histos(&lines);
    let mut v_consensus: Vec<usize> = Vec::with_capacity(v_members.len());

    // ugh, stretching tuples too far

    let mut i = 0;
    while i < v_members.len() {
        let members = v_members.get(i).unwrap();
        let answers = v_answers.get(i).unwrap();
        let mut consensus = 0;

        for val in answers.values() {
            if val == members {
                consensus += 1;
            }
        }

        v_consensus.push(consensus);
        i += 1;
    }

    let sum: usize = v_consensus.iter().sum();
    println!("ok well how about {}", sum);

    Ok(())
}

fn lines_to_histos(v: &[String]) -> (Vec<usize>, Vec<HashMap<char, usize>>) {
    let mut output = Vec::new();
    let mut group_answers: HashMap<char, usize> = HashMap::new();
    let mut group_members = Vec::new();
    let mut member_count = 0;

    for answers in v {
        if answers.is_empty() {
            group_members.push(member_count);
            output.push(group_answers.clone());

            group_answers.clear();
            member_count = 0;
        } else {
            member_count += 1;

            for c in answers.chars() {
                let count = group_answers.entry(c).or_insert(0);
                *count += 1;
            }
        }
    }

    if !group_answers.is_empty() {
        group_members.push(member_count);
        output.push(group_answers);
    }

    (group_members, output)
}
