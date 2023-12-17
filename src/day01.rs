use std::vec;

use color_eyre::eyre::Result;

use crate::solver::Answer;

pub fn solve(input: &str) -> Result<Answer> {
    let mut number_stacks: Vec<char> = vec![];
    let mut letter_stacks: Vec<char> = vec![];

    let mut part_01 = 0;
    let mut part_02 = 0;

    // part 1
    for c in input.chars() {
        if c.is_numeric() {
            // normal number
            number_stacks.push(c);
        } else if c == '\n' {
            // line termination
            add_answer(&number_stacks, &mut part_01)?;

            number_stacks.clear();
        }
    }

    number_stacks.clear();

    // part 2
    for c in input.chars() {
        if c.is_numeric() {
            // normal number
            number_stacks.push(c);
        } else if c == '\n' {
            // line termination
            add_answer(&number_stacks, &mut part_02)?;

            number_stacks.clear();
            letter_stacks.clear();
        } else if c.is_alphabetic() {
            // alphabet
            letter_stacks.push(c);

            let current_string = letter_stacks.iter().collect::<String>();

            let number = if current_string.ends_with("one") {
                Some('1')
            } else if current_string.ends_with("two") {
                Some('2')
            } else if current_string.ends_with("three") {
                Some('3')
            } else if current_string.ends_with("four") {
                Some('4')
            } else if current_string.ends_with("five") {
                Some('5')
            } else if current_string.ends_with("six") {
                Some('6')
            } else if current_string.ends_with("seven") {
                Some('7')
            } else if current_string.ends_with("eight") {
                Some('8')
            } else if current_string.ends_with("nine") {
                Some('9')
            } else {
                None
            };

            if let Some(number) = number {
                number_stacks.push(number);
            }
        }
    }

    let answer = Answer {
        part1: Some(part_01.to_string()),
        part2: Some(part_02.to_string()),
    };

    Ok(answer)
}

fn add_answer(stacks: &[char], current: &mut i32) -> Result<(), color_eyre::eyre::Error> {
    let first = stacks.first().unwrap_or(&'0');
    let last = stacks.last().unwrap_or(&'0');
    let text = format!("{}{}", first, last);

    *current += text.parse::<i32>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tracing_test::traced_test;

    use crate::{day01::solve, solver::Answer};

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        let answer = solve(input)?;
        assert_eq!(answer.part1, Some("142".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            solve("threenine\n")?,
            Answer {
                part1: Some("0".to_string()),
                part2: Some("39".to_string())
            }
        );
        assert_eq!(
            solve("eighthree\n")?,
            Answer {
                part1: Some("0".to_string()),
                part2: Some("83".to_string())
            }
        );
        assert_eq!(
            solve("nine\n")?,
            Answer {
                part1: Some("0".to_string()),
                part2: Some("99".to_string())
            }
        );

        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        let answer = solve(input)?;
        assert_eq!(answer.part2, Some("281".to_string()));

        Ok(())
    }
}
