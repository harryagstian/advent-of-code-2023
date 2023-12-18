use std::collections::{HashMap, VecDeque};

use crate::solver::Answer;

use color_eyre::eyre::Result;

use tracing::info;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Condition {
    Good,
    Bad,
    Unknown,
}

trait DisplayVecCondition {
    fn display(&self);
}

impl DisplayVecCondition for Vec<Condition> {
    fn display(&self) {
        let text = self.iter().map(|f| f.display()).collect::<String>();
        info!("{}", text);
    }
}

impl Condition {
    fn new(c: &char) -> Self {
        match c {
            '.' => Condition::Good,
            '#' => Condition::Bad,
            '?' => Condition::Unknown,
            _ => unreachable!(),
        }
    }

    fn display(&self) -> &str {
        match self {
            Condition::Good => ".",
            Condition::Bad => "#",
            Condition::Unknown => "?",
        }
    }

    fn from_line(line: &str) -> VecDeque<Self> {
        line.chars().map(|f| Self::new(&f)).collect()
    }
}

#[derive(Debug)]
struct Spring {
    raw: VecDeque<Condition>,
    valid_state: VecDeque<i64>,
}

impl Spring {
    fn new(input: &str, multiplier: usize) -> Self {
        let vec = input.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(vec.len(), 2);

        let value = std::iter::repeat(*vec.first().unwrap())
            .take(multiplier)
            .collect::<Vec<_>>()
            .join("?");
        let raw = Condition::from_line(&value);

        let valid_state = std::iter::repeat(*vec.last().unwrap())
            .take(multiplier)
            .collect::<Vec<_>>()
            .join(",")
            .split(',')
            .map(|f| f.parse::<i64>().unwrap())
            .collect();

        Self { raw, valid_state }
    }

    fn valid_count(&self) -> i64 {
        fn inner(
            condition: &VecDeque<Condition>,
            valid_state: &VecDeque<i64>,
            memo: &mut HashMap<(VecDeque<Condition>, VecDeque<i64>), i64>,
        ) -> i64 {
            // logic implemented based on https://www.youtube.com/watch?v=g3Ms5e7Jdqo
            if condition.is_empty() {
                if valid_state.is_empty() {
                    return 1;
                } else {
                    return 0;
                }
            }

            if valid_state.is_empty() {
                if condition.contains(&Condition::Bad) {
                    return 0;
                } else {
                    return 1;
                }
            }

            match memo.get(&(condition.clone(), valid_state.clone())) {
                Some(&value) => value,
                None => {
                    let mut result = 0;
                    let next_spring = *condition.front().unwrap();
                    let next_state = *valid_state.front().unwrap();

                    if next_spring == Condition::Good || next_spring == Condition::Unknown {
                        let new_condition = condition
                            .range(1..)
                            .copied()
                            .collect::<VecDeque<Condition>>();
                        result += inner(&new_condition, valid_state, memo);
                    }

                    if next_spring == Condition::Bad || next_spring == Condition::Unknown {
                        let next_good_condition_index =
                            match condition.iter().position(|f| f == &Condition::Good) {
                                Some(v) => v as i64,
                                None => i64::MAX,
                            };

                        if (next_state <= condition.len() as i64)  // there is still enough conditions to satisfy next_state number
                            && (next_state <= next_good_condition_index) // the block is at least bigger than next_state
                            // end of condition, or
                            // there is more conditions, but separated by . or ?
                            && (next_state == condition.len() as i64 || condition[next_state as usize] != Condition::Bad)
                        {
                            let new_condition = if next_state as usize + 1 > condition.len() {
                                // if block size is bigger than current vec, pass an empty vec
                                VecDeque::new()
                            } else {
                                condition
                                    .range(next_state as usize + 1..)
                                    .copied()
                                    .collect::<VecDeque<Condition>>()
                            };

                            let mut new_valid_state = valid_state.clone();
                            new_valid_state.pop_front();

                            result += inner(&new_condition, &new_valid_state, memo);
                        }
                    }
                    memo.insert((condition.clone(), valid_state.clone()), result);

                    result
                }
            }
        }

        inner(&self.raw, &self.valid_state, &mut HashMap::new())
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut answer = Answer::default();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        for (v, multiplier) in [(&mut part1, 1), (&mut part2, 5)] {
            let spring = Spring::new(line, multiplier);
            let valid_state = spring.valid_count();

            *v += valid_state;
        }
    }

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tracing_test::traced_test;

    use crate::day12::solve;

    use super::Spring;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[traced_test]
    #[test]
    fn test_valid_count() {
        let mut stacks = vec![];
        for line in TEST_INPUT.lines() {
            if line.is_empty() {
                continue;
            }
            let spring = Spring::new(line, 1);
            let valid_state = spring.valid_count();

            stacks.push(valid_state);
        }

        assert_eq!(stacks, [1, 4, 1, 1, 4, 10]);
    }

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("21".to_string()));
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("525152".to_string()));
        Ok(())
    }
}
