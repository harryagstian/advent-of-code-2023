use std::{ops::Div, str::FromStr};

use crate::{
    solver::Answer,
    utils::{Coordinate, Direction, Part},
};

use color_eyre::eyre::Result;

#[derive(Debug)]
struct Map {
    coordinates: Vec<Coordinate<i64>>,
    perimeter: i64,
}

impl Map {
    fn new(input: &str, part: Part) -> Self {
        let mut coordinates = Vec::new();
        let mut coordinate = Coordinate::new(0, 0);
        let mut perimeter = 0;

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            let vec = line.split_whitespace().collect::<Vec<&str>>();

            assert_eq!(vec.len(), 3);

            let (direction_str, steps) = match part {
                Part::One => (vec[0], vec[1].parse::<i64>().unwrap()),
                Part::Two => {
                    let mut hex_str = vec[2].to_owned();

                    hex_str = hex_str.replace(['(', ')', '#'], "");

                    let direction_str = match hex_str.chars().last().unwrap() {
                        '0' => "R",
                        '1' => "D",
                        '2' => "L",
                        '3' => "U",
                        _ => unreachable!(),
                    };

                    let steps = i64::from_str_radix(&hex_str[0..hex_str.len() - 1], 16).unwrap();

                    (direction_str, steps)
                }
            };

            let direction = Direction::from_str(direction_str).unwrap();
            let modifier = direction.get_modifier(steps as i32);

            coordinate = coordinate.add(modifier.0 as i64, modifier.1 as i64);
            coordinates.push(coordinate);

            perimeter += steps;
        }

        Self {
            coordinates,
            perimeter,
        }
    }

    fn calculate_area(&self) -> i64 {
        // reference:
        // https://en.wikipedia.org/wiki/Pick%27s_theorem
        // https://en.wikipedia.org/wiki/Shoelace_formula

        let mut area = 0;

        for index in 0..self.coordinates.len() {
            let current = self.coordinates[index];
            let next = self.coordinates[(index + 1) % self.coordinates.len()];

            area += current.x * next.y;
            area -= next.x * current.y;
        }

        area.abs().div(2) + self.perimeter.div(2) + 1
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();

    let map = Map::new(input, Part::One);
    let part1 = map.calculate_area();

    let map = Map::new(input, Part::Two);
    let part2 = map.calculate_area();

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {

    use tracing_test::traced_test;

    use super::*;
    use color_eyre::eyre::Result;

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("62".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("952408144115".to_string()));

        Ok(())
    }
}
