use std::collections::VecDeque;

use crate::solver::Answer;

use color_eyre::eyre::Result;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
enum StateError {
    #[error("out of stacks")]
    OutOfStacks,
    #[error("stacks not equal")]
    StacksNotEqual,
    #[error("valid state is not empty")]
    ValidStateNotEmpty,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            '#' => Condition::Good,
            '.' => Condition::Bad,
            '?' => Condition::Unknown,
            _ => unreachable!(),
        }
    }

    fn display(&self) -> &str {
        match self {
            Condition::Good => "#",
            Condition::Bad => "·",
            Condition::Unknown => "?",
        }
    }

    fn from_line(line: &str) -> Vec<Self> {
        line.chars().map(|f| Self::new(&f)).collect()
    }
}

#[derive(Debug)]
struct Spring {
    raw: Vec<Condition>,
    valid_state: VecDeque<i32>,
}

impl Spring {
    fn new(input: &str) -> Self {
        let vec = input.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(vec.len(), 2);

        let raw = Condition::from_line(vec.first().unwrap());
        let valid_state = vec
            .last()
            .unwrap()
            .split(',')
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        Self { raw, valid_state }
    }

    fn find_combinations(&self) -> Vec<Vec<Condition>> {
        let indices = self
            .raw
            .iter()
            .enumerate()
            .filter(|(_, &f)| f == Condition::Unknown)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        let mut stacks = Vec::from([self.raw.clone()]);
        for index in indices {
            let mut new_stacks = vec![];
            while let Some(mut current) = stacks.pop() {
                for value in [Condition::Bad, Condition::Good] {
                    current[index] = value;
                    new_stacks.push(current.clone());
                }
            }
            stacks = new_stacks;
        }

        stacks
    }

    fn is_valid(&self, combination: &Vec<Condition>) -> Result<()> {
        let mut valid_state = self.valid_state.clone();
        let mut stacks = vec![];
        for value in combination {
            // dbg!(&value, &stacks);
            match value {
                Condition::Good => {
                    stacks.push(Condition::Good);
                }
                Condition::Bad => {
                    // if stacks empty, do nothing
                    // otherwise pop_front valid_state
                    if !stacks.is_empty() {
                        let len = stacks.len() as i32;
                        let next_value = match valid_state.pop_front() {
                            Some(v) => v,
                            None => return Err(StateError::OutOfStacks.into()), // if valid_state is empty, return false
                        };

                        // if current stacks len is not equal to next state, return false
                        if len != next_value {
                            return Err(StateError::StacksNotEqual.into());
                        }

                        stacks.clear();
                    }
                }
                Condition::Unknown => unreachable!(),
            }
        }

        if !stacks.is_empty() {
            let len = stacks.len() as i32;
            let next_value = match valid_state.pop_front() {
                Some(v) => v,
                None => return Err(StateError::OutOfStacks.into()), // if valid_state is empty, return false
            };
            // if current stacks len is not equal to next state, return false
            if len != next_value {
                return Err(StateError::StacksNotEqual.into());
            }
        }

        if !valid_state.is_empty() {
            return Err(StateError::ValidStateNotEmpty.into());
        }

        Ok(())
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut answer = Answer::default();

    for line in input.lines() {
        let mut valid_state = 0;
        if line.is_empty() {
            continue;
        }
        let spring = Spring::new(line);
        let combinations = spring.find_combinations();

        for combination in combinations {
            let state = spring.is_valid(&combination);
            if state.is_ok() {
                // combination.display();
                valid_state += 1;
            }
        }

        part1 += valid_state;
    }

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use crate::day12::Condition;

    use super::Spring;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[traced_test]
    #[test]
    fn test_valid_combination() {
        let cases = vec![
            ("???.### 1,1,3", vec![("#.#.###", true)]),
            (
                "?###???????? 3,2,1",
                vec![
                    (".###.##.#...", true),
                    (".###.##..#..", true),
                    (".###.##..#.#", false),
                ],
            ),
        ];

        for (line, rest) in cases {
            let spring = Spring::new(line);
            for (condition, state) in rest {
                info!(
                    "Running test cases: Spring {}, condition: {}, state {}",
                    &line, &condition, &state
                );
                let condition_vec = Condition::from_line(condition);
                let s = spring.is_valid(&condition_vec);
                dbg!(&s);
                assert_eq!(spring.is_valid(&condition_vec).is_ok(), state);
            }
        }
    }

    #[traced_test]
    #[test]
    fn test_valid_count() {
        let mut stacks = vec![];
        for line in TEST_INPUT.lines() {
            let mut valid_state = 0;
            if line.is_empty() {
                continue;
            }
            let spring = Spring::new(line);
            let combinations = spring.find_combinations();

            for combination in combinations {
                let state = spring.is_valid(&combination);
                if state.is_ok() {
                    valid_state += 1;
                }
            }

            stacks.push(valid_state);
        }

        assert_eq!(stacks, [1, 4, 1, 1, 4, 10]);
    }
}
