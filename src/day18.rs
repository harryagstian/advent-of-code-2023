use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{
    solver::Answer,
    utils::{Coordinate, Direction},
};

use color_eyre::eyre::Result;
use tracing::info;

#[derive(Debug)]
struct Map {
    edges: HashMap<Coordinate<i32>, String>,
    boundary: ((i32, i32), (i32, i32)), // ((min_x, max_x), (min_y, max_y))
}

impl Map {
    fn new(input: &str) -> Self {
        let mut coordinate = Coordinate::new(0, 0);
        let mut edges = HashMap::new();
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let mut vec = line.split_whitespace();

            let direction_str = vec.next().unwrap();
            let step = vec.next().unwrap().parse::<usize>().unwrap();

            let direction = Direction::from_str(direction_str).unwrap();
            let modifier = direction.get_modifier();

            for _ in 0..step {
                coordinate = coordinate.add(modifier.0, modifier.1);
            }

            min_x = std::cmp::min(coordinate.x, min_x);
            max_x = std::cmp::max(coordinate.x, max_x);
            min_y = std::cmp::min(coordinate.y, min_y);
            max_y = std::cmp::max(coordinate.y, max_y);
        }

        // normalize the coordinate so that min_x / min_y >= 0

        let mod_x = if min_x < 0 { 0 - min_x } else { 0 };
        let mod_y = if min_y < 0 { 0 - min_y } else { 0 };

        min_x += mod_x;
        max_x += mod_x;
        min_y += mod_y;
        max_y += mod_y;

        coordinate = Coordinate::new(mod_x, mod_y);

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let mut vec = line.split_whitespace();

            let direction_str = vec.next().unwrap();
            let step = vec.next().unwrap().parse::<usize>().unwrap();
            let color = vec.next().unwrap();

            let direction = Direction::from_str(direction_str).unwrap();
            let modifier = direction.get_modifier();

            for _ in 0..step {
                coordinate = coordinate.add(modifier.0, modifier.1);
                edges.insert(coordinate, color.to_string());
            }
        }

        Self {
            edges,
            boundary: ((min_x, max_x), (min_y, max_y)),
        }
    }

    fn display(&self, seen: &HashSet<Coordinate<i32>>) {
        let ((min_x, max_x), (min_y, max_y)) = self.boundary;
        let mut vec = vec![];

        for y in min_y..=max_y {
            let mut lines = String::new();

            for x in min_x..=max_x {
                let coordinate = Coordinate::new(x, y);

                let value = if self.edges.contains_key(&coordinate) || seen.contains(&coordinate) {
                    '#'
                } else {
                    '·'
                };
                lines.push(value);
            }
            vec.push(lines);
        }

        vec.reverse();
        info!("\n{}", vec.join("\n"));
    }

    fn floodfill(&self) -> Option<usize> {
        fn recurse(
            map: &Map,
            coordinate: &Coordinate<i32>,
            seen: &mut HashSet<Coordinate<i32>>,
        ) -> bool {
            let ((min_x, max_x), (min_y, max_y)) = map.boundary;

            if coordinate.x < min_x
                || coordinate.x > max_x
                || coordinate.y < min_y
                || coordinate.y > max_y
            {
                return false;
            }

            if seen.contains(coordinate) {
                return true;
            }

            seen.insert(*coordinate);

            if map.edges.contains_key(coordinate) {
                return true;
            }

            for direction in [
                Direction::Up,
                Direction::Left,
                Direction::Right,
                Direction::Down,
            ] {
                let modifier = direction.get_modifier();
                let new_coordinate = coordinate.add(modifier.0, modifier.1);

                match recurse(map, &new_coordinate, seen) {
                    true => (),
                    false => return false,
                }
            }

            true
        }

        for coordinate in self.edges.keys() {
            for direction in [
                Direction::Up,
                Direction::Left,
                Direction::Right,
                Direction::Down,
            ] {
                let modifier = direction.get_modifier();
                let new_coordinate = coordinate.add(modifier.0, modifier.1);
                let mut seen = HashSet::new();
                if !self.edges.contains_key(&new_coordinate) {
                    let found = recurse(self, &new_coordinate, &mut seen);

                    if found && !seen.is_empty() {
                        for key in self.edges.keys() {
                            seen.insert(*key);
                        }

                        self.display(&seen);
                        return Some(seen.len());
                    }
                }
            }
        }

        None
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let part2 = 0;
    let mut answer = Answer::default();

    let map = Map::new(input);
    dbg!(&map);
    map.display(&HashSet::new());
    let part1 = map.floodfill().unwrap();
    dbg!(&part1);

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
    fn test_map_edges_all_positive() {
        let map = Map::new(TEST_INPUT);
        let all_positive = map.edges.iter().all(|(f, _)| f.x >= 0 && f.y >= 0);

        assert!(all_positive);
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("".to_string()));

        Ok(())
    }
}
