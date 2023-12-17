use std::collections::{HashMap, HashSet};

use crate::solver::Answer;

use color_eyre::eyre::Result;
use tracing::info;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum SpaceItem {
    Galaxy,
    Empty,
}

impl SpaceItem {
    fn new(c: &char) -> Self {
        match c {
            '#' => SpaceItem::Galaxy,
            '.' => SpaceItem::Empty,
            _ => unreachable!(),
        }
    }

    fn display(&self) -> &str {
        match self {
            SpaceItem::Galaxy => "#",
            SpaceItem::Empty => "·",
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct SpaceObjects {
    y: HashSet<i64>,
    x: HashSet<i64>,
    coordinates: HashMap<i64, Coordinate>,
}

#[derive(Debug)]
struct Image {
    map: Vec<Vec<SpaceItem>>,
    space_objects: SpaceObjects,
}

impl Image {
    fn new(input: &str) -> Self {
        let mut map = vec![];
        let mut space_objects = SpaceObjects {
            y: HashSet::new(),
            x: HashSet::new(),
            coordinates: HashMap::new(),
        };
        let mut lines = input.lines().collect::<Vec<&str>>();
        lines.reverse();

        for y_row in lines {
            if y_row.is_empty() {
                continue;
            }

            let mut line_vec = vec![];
            for value in y_row.chars() {
                let item = SpaceItem::new(&value);
                if item == SpaceItem::Galaxy {
                    let coordinate = Coordinate {
                        x: line_vec.len() as i64,
                        y: map.len() as i64,
                    };
                    space_objects.y.insert(coordinate.y);
                    space_objects.x.insert(coordinate.x);
                    space_objects
                        .coordinates
                        .insert(space_objects.coordinates.len() as i64 + 1, coordinate);
                }
                line_vec.push(item);
            }

            map.push(line_vec);
        }

        Self { map, space_objects }
    }

    fn display(&self) {
        let mut text = "\n".to_string();
        let mut map = self.map.clone();
        assert!(!map.is_empty());

        map.reverse(); // reverse back so that map prints like in the website

        for line in map.iter() {
            for c in line {
                text.push_str(c.display());
            }
            text.push('\n');
        }

        info!("{}", text);
    }

    fn solve(&self, expansion_factor: i64) -> i64 {
        let mut distance = 0;
        let mut iterator = self.space_objects.coordinates.keys().collect::<Vec<&i64>>();
        iterator.sort();

        for left_index in &iterator {
            for right_index in &iterator {
                if left_index >= right_index {
                    continue;
                }
                let start_coordinate = self.space_objects.coordinates.get(left_index).unwrap();
                let end_coordinate = self.space_objects.coordinates.get(right_index).unwrap();

                let get_distance =
                    self.get_distance(start_coordinate, end_coordinate, expansion_factor);
                distance += get_distance;
            }
        }
        distance
    }

    fn get_distance(
        &self,
        start_coordinate: &Coordinate,
        end_coordinate: &Coordinate,
        expansion_factor: i64,
    ) -> i64 {
        let x_distance = self.distance_between_point(
            start_coordinate.x,
            end_coordinate.x,
            &self.space_objects.x,
            expansion_factor,
        );

        let y_distance = self.distance_between_point(
            start_coordinate.y,
            end_coordinate.y,
            &self.space_objects.y,
            expansion_factor,
        );

        x_distance + y_distance
    }

    fn distance_between_point(
        &self,
        start: i64,
        end: i64,
        set: &HashSet<i64>,
        expansion_factor: i64,
    ) -> i64 {
        assert!(expansion_factor > 1);
        let mut distance = 0;
        let min = std::cmp::min(start, end);
        let max = std::cmp::max(start, end);

        for value in min..max {
            distance += if !set.contains(&value) {
                expansion_factor
            } else {
                1
            };
        }

        distance
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();
    let image = Image::new(input);
    image.display();

    let part1 = image.solve(2);
    let part2 = image.solve(1000000);

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {

    use tracing::info;
    use tracing_test::traced_test;

    use crate::day11::{Coordinate, Image};

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[traced_test]
    #[test]
    fn test_image_get_distance() {
        let image = Image::new(TEST_INPUT);
        image.display();
        let items = Vec::from([
            (Coordinate { x: 1, y: 4 }, Coordinate { x: 4, y: 0 }, 9), // 5 to 9
            (Coordinate { x: 3, y: 9 }, Coordinate { x: 7, y: 1 }, 15), // 1 to 7
            (Coordinate { x: 9, y: 3 }, Coordinate { x: 0, y: 7 }, 17), // 3 to 6
            (Coordinate { x: 0, y: 0 }, Coordinate { x: 4, y: 0 }, 5), // 8 to 9
        ]);

        for (start, end, target_distance) in items {
            info!(
                "Testing cases start: {:?}, end {:?}, target distance {}",
                &start, &end, &target_distance
            );
            assert!(image
                .space_objects
                .coordinates
                .values()
                .any(|f| f == &start));
            assert!(image.space_objects.coordinates.values().any(|f| f == &end));
            let distance = image.get_distance(&start, &end, 2);

            assert_eq!(distance, target_distance);
        }
    }

    #[traced_test]
    #[test]
    fn test_part1() {
        let image = Image::new(TEST_INPUT);
        image.display();

        let distance = image.solve(2);
        assert_eq!(distance, 374);
    }

    #[traced_test]
    #[test]
    fn test_part2() {
        let image = Image::new(TEST_INPUT);
        image.display();

        let distance = image.solve(10);
        assert_eq!(distance, 1030);

        let distance = image.solve(100);
        assert_eq!(distance, 8410);
    }
}
