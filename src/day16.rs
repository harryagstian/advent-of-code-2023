use std::collections::{HashMap, HashSet};

use crate::{solver::Answer, utils::Coordinate};

use color_eyre::eyre::Result;
use strum::IntoEnumIterator;
use tracing::info;

use crate::utils::Direction;

#[derive(Debug, Clone, Copy)]
enum Node {
    Empty,
    Mirror(char),
    Splitter(char),
}

impl Node {
    fn from_char(c: char) -> Self {
        match c {
            '/' | '\\' => Self::Mirror(c),
            '|' | '-' => Self::Splitter(c),
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }

    fn get_direction_pair(&self) -> HashMap<Direction, Vec<Direction>> {
        let pairs = match self {
            Node::Mirror(c) => match c {
                '/' => [
                    (Direction::Up, vec![Direction::Right]),
                    (Direction::Right, vec![Direction::Up]),
                    (Direction::Down, vec![Direction::Left]),
                    (Direction::Left, vec![Direction::Down]),
                ],
                '\\' => [
                    (Direction::Up, vec![Direction::Left]),
                    (Direction::Left, vec![Direction::Up]),
                    (Direction::Down, vec![Direction::Right]),
                    (Direction::Right, vec![Direction::Down]),
                ],
                _ => unreachable!(),
            },
            Node::Splitter(c) => match c {
                '-' => [
                    (Direction::Left, vec![Direction::Left]),
                    (Direction::Right, vec![Direction::Right]),
                    (Direction::Up, vec![Direction::Left, Direction::Right]),
                    (Direction::Down, vec![Direction::Left, Direction::Right]),
                ],
                '|' => [
                    (Direction::Up, vec![Direction::Up]),
                    (Direction::Down, vec![Direction::Down]),
                    (Direction::Left, vec![Direction::Up, Direction::Down]),
                    (Direction::Right, vec![Direction::Up, Direction::Down]),
                ],
                _ => unreachable!(),
            },
            Node::Empty => [
                (Direction::Left, vec![Direction::Left]),
                (Direction::Right, vec![Direction::Right]),
                (Direction::Down, vec![Direction::Down]),
                (Direction::Up, vec![Direction::Up]),
            ],
        };

        pairs.into_iter().collect()
    }

    fn get_next_direction(&self, direction: &Direction) -> Vec<Direction> {
        let pairs = self.get_direction_pair();

        pairs.get(direction).unwrap().clone()
    }

    fn display(&self) -> &str {
        match self {
            Node::Empty => "·",
            Node::Mirror(c) | Node::Splitter(c) => match c {
                '/' => "╱",
                '\\' => "╲",
                '|' => "│",
                '-' => "━",
                _ => unreachable!(),
            },
        }
    }
}

struct Grid {
    map: Vec<Vec<Node>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut map = vec![];
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            let mut line_vec = vec![];
            for c in line.chars() {
                let node = Node::from_char(c);
                line_vec.push(node);
            }

            map.push(line_vec);
        }

        map.reverse();

        Self { map }
    }

    fn display(&self, traveled: HashSet<Coordinate<i32>>) {
        let mut text = "\n".to_string();

        let map = self.map.clone();
        // map.reverse();

        for (y_index, y_row) in map.iter().enumerate() {
            for (x_index, value) in y_row.iter().enumerate() {
                let coordinate = Coordinate::new(x_index as i32, y_index as i32);
                let t = if traveled.contains(&coordinate) {
                    "#"
                } else {
                    value.display()
                };

                text.push_str(t);
            }

            text.push('\n');
        }

        info!("{}", text);
    }

    fn travel(
        &self,
        initial_coordinate: Coordinate<i32>,
        initial_direction: Direction,
    ) -> HashSet<Coordinate<i32>> {
        let mut queue = vec![(initial_coordinate, initial_direction)];
        let mut traveled = HashSet::new();
        let mut cache = HashSet::new(); // prevent forever-loop

        let max_y = self.map.len();
        let max_x = self.map[0].len();

        while let Some((current_coordinate, current_direction)) = queue.pop() {
            let (mod_x, mod_y) = current_direction.get_modifier();
            let next_coordinate = current_coordinate.add(mod_x, mod_y);

            // OOB
            if next_coordinate.x < 0
                || next_coordinate.y < 0
                || next_coordinate.x >= max_x as i32
                || next_coordinate.y >= max_y as i32
            {
                continue;
            };

            if cache.contains(&(next_coordinate, current_direction)) {
                continue;
            } else {
                cache.insert((next_coordinate, current_direction));
                traveled.insert(next_coordinate);
            }

            let next_node = &self.map[next_coordinate.y as usize][next_coordinate.x as usize];

            let next_directions = next_node.get_next_direction(&current_direction);

            for next_direction in next_directions {
                queue.push((next_coordinate, next_direction));
            }
        }

        traveled
    }

    fn maximum_energized(&self) -> i32 {
        let max_x = self.map[0].len() as i32;
        let max_y = self.map.len() as i32;
        let mut max = 0;

        let mut stacks = vec![];
        for initial_direction in Direction::iter() {
            match initial_direction {
                Direction::Up => {
                    for i in 0..max_x {
                        stacks.push((initial_direction, (i, -1)));
                    }
                }
                Direction::Down => {
                    for i in 0..max_x {
                        stacks.push((initial_direction, (i, max_y)));
                    }
                }
                Direction::Right => {
                    for i in 0..max_y {
                        stacks.push((initial_direction, (-1, i)));
                    }
                }
                Direction::Left => {
                    for i in 0..max_y {
                        stacks.push((initial_direction, (max_x, i)));
                    }
                }
                _ => continue,
            }
        }

        for (initial_direction, initial_coordinate_raw) in stacks {
            let initial_coordinate =
                Coordinate::new(initial_coordinate_raw.0, initial_coordinate_raw.1);
            let traveled = self.travel(initial_coordinate, initial_direction);

            max = std::cmp::max(max, traveled.len() as i32);
        }

        max
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();

    let grid = Grid::new(input);
    grid.display(HashSet::new());

    let traveled = grid.travel(
        Coordinate::new(-1, grid.map.len() as i32 - 1),
        Direction::Right,
    );
    let part1 = traveled.len();
    info!("Part 1");
    grid.display(traveled);

    info!("Part 2");
    let part2 = grid.maximum_energized();

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {
    
    use tracing_test::traced_test;

    use super::*;
    use color_eyre::eyre::Result;

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("46".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("51".to_string()));

        Ok(())
    }
}
