use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use crate::{
    solver::Answer,
    utils::{Coordinate, Direction, Part},
};

use color_eyre::eyre::Result;
use tracing::info;

struct Map {
    data: Vec<Vec<i32>>,
}

trait PriorityQueue {
    fn priority_push(&mut self, new_queue: Queue);
}

#[derive(Debug, Eq)]
struct Queue {
    coordinate: Coordinate<i32>,
    previous_direction: Direction,
    steps_in_this_direction: i32,
    heat_loss: i32,
    visited: HashSet<(Coordinate<i32>, Direction, i32)>,
}

impl PartialEq for Queue {
    fn eq(&self, other: &Self) -> bool {
        self.visited.len() == other.visited.len() && self.heat_loss == other.heat_loss
    }
}

impl PartialOrd for Queue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Queue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss
            .cmp(&other.heat_loss)
            .then_with(|| self.visited.len().cmp(&other.visited.len()))
    }
}

impl PriorityQueue for VecDeque<Queue> {
    fn priority_push(&mut self, new_queue: Queue) {
        let position = self.iter().position(|f| f > &new_queue);

        match position {
            Some(index) => self.insert(index, new_queue),
            None => self.push_back(new_queue),
        }
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let mut data = vec![];

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            let row = line
                .chars()
                .map(|f| f.to_digit(10).unwrap() as i32)
                .collect();
            data.push(row);
        }

        data.reverse();

        Self { data }
    }

    fn display(&self, queue: Option<&Queue>) {
        let mut text = "\n".to_string();

        let mut set = HashMap::new();
        if let Some(queue) = queue {
            queue.visited.iter().for_each(|f| {
                set.insert(f.0, f.1);
            });
        };

        for y_index in (0..self.data.len()).rev() {
            for x_index in 0..self.data[0].len() {
                let coordinate = Coordinate::new(x_index as i32, y_index as i32);

                let value = if let Some(value) = set.get(&coordinate) {
                    value.display().to_owned()
                } else {
                    self.data[y_index][x_index].to_string()
                };

                text.push_str(&value);
            }
            text.push('\n');
        }

        info!("{}", text);
    }

    fn travel(
        &self,
        initial_coordinate: Coordinate<i32>,
        target_coordinate: Coordinate<i32>,
        part: Part,
    ) -> Option<i32> {
        let mut stacks = VecDeque::new();
        let mut visited = HashSet::new();

        let max_y = self.data.len() as i32;
        let max_x = self.data[0].len() as i32;

        // initially fill up stacks
        for direction in [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ] {
            let modifier = direction.get_modifier(1);
            let next_coordinate = initial_coordinate.add(modifier.0, modifier.1);

            if next_coordinate.x < 0
                || next_coordinate.y < 0
                || next_coordinate.x >= max_x
                || next_coordinate.y >= max_y
            {
                continue;
            }

            let next_heat_loss = self.data[next_coordinate.y as usize][next_coordinate.x as usize];
            let queue = Queue {
                coordinate: next_coordinate,
                previous_direction: direction,
                steps_in_this_direction: 1,
                heat_loss: next_heat_loss,
                visited: HashSet::from([(next_coordinate, direction, next_heat_loss)]),
            };

            stacks.priority_push(queue);
        }

        while let Some(current_queue) = stacks.pop_front() {
            if current_queue.coordinate == target_coordinate {
                if part == Part::Two && current_queue.steps_in_this_direction < 4 {
                    continue;
                }

                self.display(Some(&current_queue));
                return Some(current_queue.heat_loss);
            }

            if visited.contains(&(
                current_queue.coordinate,
                current_queue.previous_direction,
                current_queue.steps_in_this_direction,
            )) {
                continue;
            }

            visited.insert((
                current_queue.coordinate,
                current_queue.previous_direction,
                current_queue.steps_in_this_direction,
            ));

            for next_direction in [
                Direction::Up,
                Direction::Down,
                Direction::Right,
                Direction::Left,
            ] {
                if next_direction == current_queue.previous_direction.reverse() {
                    // cannot go in reverse
                    continue;
                }

                let straight_limit = match part {
                    Part::One => 3,
                    Part::Two => 10,
                };

                let mut next_steps_in_this_direction = 1;
                let modifier = next_direction.get_modifier(1);
                let next_coordinate = current_queue.coordinate.add(modifier.0, modifier.1);

                if next_coordinate.x < 0
                    || next_coordinate.y < 0
                    || next_coordinate.x >= max_x
                    || next_coordinate.y >= max_y
                {
                    continue;
                }

                let next_heat_loss = current_queue.heat_loss
                    + self.data[next_coordinate.y as usize][next_coordinate.x as usize];

                if current_queue.previous_direction == next_direction {
                    if current_queue.steps_in_this_direction == straight_limit {
                        // cannot go straight more than 3 or 10 times
                        continue;
                    }

                    next_steps_in_this_direction = current_queue.steps_in_this_direction + 1;
                }

                if part == Part::Two
                    && current_queue.previous_direction != next_direction
                    && current_queue.steps_in_this_direction < 4
                {
                    // need to go at least 4 times straight
                    continue;
                }

                if visited.contains(&(
                    next_coordinate,
                    next_direction,
                    next_steps_in_this_direction,
                )) {
                    continue;
                }

                let mut next_visited = current_queue.visited.clone();
                next_visited.insert((
                    next_coordinate,
                    next_direction,
                    next_steps_in_this_direction,
                ));

                let next_queue = Queue {
                    coordinate: next_coordinate,
                    previous_direction: next_direction,
                    steps_in_this_direction: next_steps_in_this_direction,
                    heat_loss: next_heat_loss,
                    visited: HashSet::new(),
                };

                stacks.priority_push(next_queue);
            }
        }

        None
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();

    let map = Map::new(input);
    map.display(None);

    let part1 = map
        .travel(
            Coordinate::new(0, map.data.len() as i32 - 1),
            Coordinate::new(map.data[0].len() as i32 - 1, 0),
            Part::One,
        )
        .unwrap();

    let part2 = map
        .travel(
            Coordinate::new(0, map.data.len() as i32 - 1),
            Coordinate::new(map.data[0].len() as i32 - 1, 0),
            Part::Two,
        )
        .unwrap();

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;
    use color_eyre::eyre::Result;

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[traced_test]
    #[test]
    fn test_priority_push() {
        fn create_queues(items: &[i32]) -> VecDeque<Queue> {
            let mut queues = VecDeque::new();

            for item in items {
                let new_queue = Queue {
                    coordinate: Coordinate::new(0, 0),
                    previous_direction: Direction::Up,
                    steps_in_this_direction: 0,
                    heat_loss: *item,
                    visited: HashSet::new(),
                };
                queues.priority_push(new_queue)
            }

            queues
        }

        let mut items = vec![100, 20, 50, 20, 30, 0, 20, -5, 0];

        let queues = create_queues(&items);

        let result = queues.iter().map(|f| f.heat_loss).collect::<Vec<_>>();

        items.sort();
        assert_eq!(items, result);
    }

    #[traced_test]
    #[test]
    fn test_part1() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part1, Some("102".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2() -> Result<()> {
        let answer = solve(TEST_INPUT)?;

        assert_eq!(answer.part2, Some("94".to_string()));

        Ok(())
    }
}
