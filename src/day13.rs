use crate::{
    solver::Answer,
    utils::{get_column, get_row},
};

use color_eyre::eyre::Result;
use tracing::info;

#[derive(Debug, Clone)]
struct Pattern {
    map: Vec<Vec<char>>,
}

impl Pattern {
    fn new(input: &str) -> Self {
        let mut map = vec![];
        for line in input.lines() {
            map.push(line.chars().collect::<Vec<_>>());
        }

        // 1 starts from top left, we don't need to do map.reverse()

        Self { map }
    }

    fn line_diff_with_autofix(
        left_slice: &[char],
        right_slice: &[char],
        can_autofix_init: bool,
    ) -> (bool, bool) // returns (is identical or not), (is autofixed used or not)
    {
        assert_eq!(left_slice.len(), right_slice.len());
        let len = left_slice.len();
        let mut can_autofix = can_autofix_init;

        for i in 0..len {
            let left = left_slice[i];
            let right = right_slice[i];

            if left != right {
                if can_autofix {
                    can_autofix = false;
                    continue;
                }

                return (false, can_autofix_init);
            }
        }

        (true, can_autofix)
    }

    fn check_reflection<F>(
        map: &[Vec<char>],
        len: usize,
        get_element: F,
        smudge_init: bool,
    ) -> Option<i32>
    where
        F: Fn(&[Vec<char>], i32) -> Option<Vec<char>>,
    {
        for i in 0..len - 1 {
            let mut smudge = smudge_init;
            let mut left_index = i as i32;
            let mut right_index = i as i32 + 1;

            let left = get_element(map, left_index).unwrap();
            let right = get_element(map, right_index).unwrap();

            let t = Self::line_diff_with_autofix(&left, &right, smudge);
            let equal_line = t.0;
            smudge = t.1;

            if equal_line {
                let mut is_reflection = true;
                loop {
                    left_index -= 1;
                    right_index += 1;

                    let left_opt = get_element(map, left_index);
                    let right_opt = get_element(map, right_index);

                    match (left_opt, right_opt) {
                        (Some(left), Some(right)) => {
                            let t = Self::line_diff_with_autofix(&left, &right, smudge);
                            let equal_line = t.0;
                            smudge = t.1;

                            if !equal_line {
                                is_reflection = false;
                                break;
                            }
                        }
                        _ => break,
                    }
                }

                if is_reflection && (!smudge_init || !smudge) {
                    return Some(i as i32);
                }
            }
        }
        None
    }

    fn get_reflection_value(&self, smudge: bool) -> i32 {
        let max_column = self.map[0].len();
        let max_row = self.map.len();

        let column = Self::check_reflection(&self.map, max_column, get_column, smudge);

        if let Some(value) = column {
            value + 1
        } else {
            let row = Self::check_reflection(&self.map, max_row, get_row, smudge);
            (row.unwrap() + 1) * 100
        }
    }

    fn display(&self) {
        let mut text = "\n".to_string();

        for y_row in &self.map {
            text.push_str(&y_row.iter().collect::<String>());
            text.push('\n');
        }

        info!("{}", text);
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut answer = Answer::default();
    let mut stacks = vec![];

    fn create_pattern(stacks: &mut Vec<&str>) -> (i32, i32) {
        let pattern = Pattern::new(&stacks.join("\n"));
        pattern.display();
        let p1 = pattern.get_reflection_value(false);
        let p2 = pattern.get_reflection_value(true);

        stacks.clear();
        (p1, p2)
    }

    for line in input.lines() {
        if line.is_empty() {
            let (p1, p2) = create_pattern(&mut stacks);
            part1 += p1;
            part2 += p2
        } else {
            stacks.push(line);
        }
    }

    let (p1, p2) = create_pattern(&mut stacks);
    part1 += p1;
    part2 += p2;

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use tracing_test::traced_test;

    use crate::day13::solve;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("405".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("400".to_string()));

        Ok(())
    }
}
