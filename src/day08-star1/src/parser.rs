#[path = "test_data.rs"]
mod test_data;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum Operator {
    Acc,
    Jmp,
    NoOp,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Instruction {
    operator: Operator,
    operand: i32,
}

////////////////////////////////////////////////////////////////////////////////

impl Instruction {
    pub fn parse(lines: &[String]) -> Result<Option<Vec<Instruction>>, String> {
        if lines.is_empty() {
            return Ok(None);
        }

        let mut v = Vec::new();

        for line in lines {
            let pieces: Vec<&str> = line.split(' ').collect();

            let operator = match pieces.get(0) {
                Some(&"nop") => Operator::NoOp,
                Some(&"acc") => Operator::Acc,
                Some(&"jmp") => Operator::Jmp,
                _ => return Err(format!("{} is bad", line)),
            };

            let operand = match pieces.get(1) {
                Some(s) => match s.parse::<i32>() {
                    Ok(i) => i,
                    Err(e) => return Err(format!("{} well there's no parsing that operand", e)),
                },
                None => return Err(format!("{} has no operand?", line)),
            };

            v.push(Instruction { operator, operand });
        }

        Ok(Some(v))
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = test_data::read_test_data("day08-star1/micro.txt").unwrap();
        let data = Instruction::parse(&lines);

        assert_matches!(data, Ok(_));

        if let Ok(result) = data {
            assert_matches!(result, Some(_));

            if let Some(instructions) = result {
                assert_matches!(instructions.get(0), Some(_));

                if let Some(inst) = instructions.get(0) {
                    assert_matches!(inst.operator, Operator::NoOp);
                    assert_eq!(inst.operand, 0);
                }
            }
        }
    }
}
