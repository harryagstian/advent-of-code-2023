use std::{
    collections::{HashMap, HashSet},
    iter,
};

use crate::{
    solver::Answer,
    utils::{get_column, get_row, update_column, update_row, Direction},
};

use color_eyre::eyre::Result;
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Item {
    RoundRock,
    CubeRock,
    Empty,
}

impl Item {
    fn new(input: &char) -> Self {
        match input {
            '#' => Self::CubeRock,
            'O' => Self::RoundRock,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }

    fn display(&self) -> &str {
        match self {
            Item::CubeRock => "#",
            Item::RoundRock => "O",
            Item::Empty => ".",
        }
    }
}

#[derive(Debug)]
struct Platform {
    map: Vec<Vec<Item>>,
}

impl Platform {
    fn new(input: &str) -> Self {
        let mut map = vec![];

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            let mut line_vec = vec![];
            for c in line.chars() {
                line_vec.push(Item::new(&c));
            }

            map.push(line_vec);
        }

        Self { map }
    }

    fn display(&self) {
        let mut text = "\n".to_string();

        for y_row in &self.map {
            text.push_str(&y_row.iter().map(|f| f.display()).collect::<String>());
            text.push('\n');
        }

        info!("{}", text);
    }

    fn as_string(&self) -> String {
        let mut text = String::new();
        for y_row in &self.map {
            text.push_str(&y_row.iter().map(|f| f.display()).collect::<String>());
        }

        text
    }

    fn tilt(&mut self, direction: &Direction) {
        let (len, get_elements, update_elements) = match direction.is_horizontal() {
            false => (
                // column wise
                self.map[0].len(),
                Box::new(get_column::<Item>) as Box<dyn Fn(&[Vec<_>], i32) -> Option<Vec<_>>>,
                Box::new(update_column::<Item>) as Box<dyn Fn(&mut [Vec<_>], &[_], i32, bool)>,
            ),
            true => (
                // row wise
                self.map.len(),
                Box::new(get_row::<Item>) as Box<dyn Fn(&[Vec<_>], i32) -> Option<Vec<_>>>,
                Box::new(update_row::<Item>) as Box<dyn Fn(&mut [Vec<_>], &[_], i32, bool)>,
            ),
        };

        let should_reverse = match direction {
            // because we move RoundRock to front of the vec, South and East need to be reversed
            Direction::North | Direction::West => false,
            Direction::South | Direction::East => true,
        };

        for index in 0..len {
            let mut elements: Vec<Item> = get_elements(&self.map, index as i32).unwrap();
            let mut new_elements = vec![];

            let mut round_count = 0;
            let mut empty_count = 0;

            if should_reverse {
                elements.reverse();
            }

            for current in &elements {
                match current {
                    Item::RoundRock => round_count += 1,
                    Item::CubeRock => {
                        if round_count > 0 {
                            new_elements.extend(iter::repeat(Item::RoundRock).take(round_count));
                            round_count = 0;
                        }

                        if empty_count > 0 {
                            new_elements.extend(iter::repeat(Item::Empty).take(empty_count));
                            empty_count = 0;
                        }

                        new_elements.push(Item::CubeRock);
                    }
                    Item::Empty => empty_count += 1,
                }
            }

            if round_count > 0 {
                new_elements.extend(iter::repeat(Item::RoundRock).take(round_count));
            }

            if empty_count > 0 {
                new_elements.extend(iter::repeat(Item::Empty).take(empty_count));
            }

            update_elements(&mut self.map, &new_elements, index as i32, should_reverse);
        }
    }

    fn get_weight(&self) -> i32 {
        let mut result = 0;
        let len = self.map.len();

        for (index, row) in self.map.iter().enumerate() {
            let round_count = row.iter().filter(|&f| f == &Item::RoundRock).count();
            let value = round_count * (len - index);

            result += value;
        }

        result as i32
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut part1 = 0;
    let mut answer = Answer::default();

    let mut platform = Platform::new(input);
    platform.display();

    let mut current_cycle = 0;
    let max_cycle = 1000000000;

    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut cache: HashMap<String, Vec<usize>> = HashMap::new();

    while current_cycle < max_cycle {
        for direction in &directions {
            platform.tilt(direction);

            if current_cycle == 0 && direction == &Direction::North {
                part1 = platform.get_weight();
            }
        }

        let key = platform.as_string();

        if let Some(vec) = cache.get_mut(&key) {
            vec.push(current_cycle);

            if vec.len() > 4 {
                let diff: HashSet<usize> =
                    vec.windows(2).map(|window| window[1] - window[0]).collect();

                if diff.len() == 1 {
                    let range = max_cycle - current_cycle;
                    let diff = *diff.iter().next().unwrap();
                    let multiplier = num::Integer::div_floor(&range, &diff);

                    current_cycle += diff * multiplier;

                    assert!(current_cycle < max_cycle);
                }
            }
        } else {
            cache.insert(key, vec![current_cycle]);
        };

        current_cycle += 1;
    }

    let part2 = platform.get_weight();

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use color_eyre::eyre::Result;

    use crate::{
        day14::{solve, Platform},
        utils::Direction,
    };

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("136".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("64".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_platform_tilt() {
        let pairs = [
            (Direction::North, "OOOO.#.O..OO..#....#OO..O##..OO..#.OO...........#...#....#.#..O..#.O.O..O.......#....###..#....#...."),
            (Direction::West, "O....#....OOO.#....#.....##...OO.#OO....OO......#.O.#O...#.#O....#OO..O.........#....###..#OO..#...."),
            (Direction::South, ".....#........#....#...O.##......#......O.O....O#OO.#..O.#.#O....#....OO....OO..#OO..###..#OO.O#...O"),
            (Direction::East, "....O#.....OOO#....#.....##....OO#....OO......OO#..O#...O#.#....O#..OO.........O#....###..#..OO#...."),
        ];

        let platform = Platform::new(TEST_INPUT);
        platform.display();

        for (direction, expected_output) in pairs {
            info!("Running test for direction {:?}", direction);
            let mut platform = Platform::new(TEST_INPUT);

            platform.tilt(&direction);
            platform.display();

            assert_eq!(&platform.as_string(), expected_output);
        }
    }
}
