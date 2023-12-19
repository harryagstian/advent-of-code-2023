use crate::solver::Answer;

use color_eyre::eyre::Result;
use tracing::info;

pub fn solve(input: &str) -> Result<Answer> {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut answer = Answer::default();

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use super::*;
    use color_eyre::eyre::Result;

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

        assert_eq!(answer.part2, Some("".to_string()));

        Ok(())
    }
}
