use std::collections::HashSet;

use color_eyre::eyre::Result;

use crate::solver::Answer;

struct Sequence {
    values: Vec<i32>,
}

impl Sequence {
    fn new(input: &str) -> Self {
        let values = input
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();

        Self { values }
    }

    fn get_previous_value(&self) -> i32 {
        let mut values = self.values.clone();
        values.reverse();
        Self::get_next_value_internal(&values)
    }

    fn get_next_value(&self) -> i32 {
        Self::get_next_value_internal(&self.values)
    }

    fn get_next_value_internal(values: &Vec<i32>) -> i32 {
        let mut diffs = vec![];
        let mut diffs_set = HashSet::new();

        for index in 0..values.len() - 1 {
            let current = values[index];
            let next = values[index + 1];
            let diff = next - current;

            diffs_set.insert(diff);
            diffs.push(diff);
        }

        let next_diff = if diffs_set.len() > 1 {
            Self::get_next_value_internal(&diffs)
        } else {
            diffs.pop().unwrap()
        };

        values.last().unwrap() + next_diff
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let sequence = Sequence::new(line);
        let next = sequence.get_next_value();
        part1 += next;

        let previous = sequence.get_previous_value();
        part2 += previous;

        // dbg!(&line, &next);
    }

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tracing_test::traced_test;

    use crate::day09::solve;
    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("114".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("2".to_string()));

        Ok(())
    }
}
