use color_eyre::eyre::Result;

use crate::solver::Answer;

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn get_win_possibilities(&self) -> u64 {
        (0..=self.time)
            .filter(|&i| i * (self.time - i) > self.distance)
            .count() as u64
    }
}

pub fn solve_day06(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();
    let mut part1 = 1;

    let mut time_vec = vec![];
    let mut distance_vec = vec![];

    for (index, line) in input.lines().enumerate() {
        match index {
            0 => {
                insert_to_vec(line, &mut time_vec);
            }
            1 => {
                insert_to_vec(line, &mut distance_vec);
            }
            _ => break,
        }
    }

    assert_eq!(time_vec.len(), distance_vec.len());

    for index in 0..time_vec.len() {
        let time = time_vec[index];
        let distance = distance_vec[index];

        let race = Race::new(time, distance);
        part1 *= race.get_win_possibilities();
    }

    let time = time_vec
        .iter()
        .map(|f| f.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = distance_vec
        .iter()
        .map(|f| f.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let race = Race::new(time, distance);
    let part2 = race.get_win_possibilities();

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());

    Ok(answer)
}

fn insert_to_vec(line: &str, time_vec: &mut Vec<u64>) {
    let vec = line.split(':').collect::<Vec<&str>>();
    assert_eq!(vec.len(), 2);
    let value = vec.last().unwrap();
    *time_vec = value
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::solve_day06;
    use color_eyre::eyre::Result;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve_day06(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("288".to_string()));

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve_day06(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("71503".to_string()));

        Ok(())
    }
}
