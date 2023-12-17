use std::collections::HashMap;

use color_eyre::eyre::Result;

use tracing::info;

use crate::solver::Answer;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    // determine whether we can travel to the next tile
    // returns tuples:
    // - can travel to next tile or not
    // - what's the next direction
    // - what turn that we take (right / left)
    fn can_travel_to(&self, dst: Tile) -> (bool, Option<Direction>, Option<Direction>) {
        let inverted_direction = self.get_inverted();
        let r = match dst {
            Tile::Ground => (false, None, None), // cannot travel to ground
            Tile::StartingPoint => (true, None, None), // can travel to starting point, but can't go further
            Tile::Pipe(_) => {
                let pair = dst.get_direction_pair();

                // determine which direction we should go next
                if let Some(next_direction) = pair
                    .iter()
                    .filter(|&f| f != &inverted_direction)
                    .copied()
                    .next()
                {
                    let turning_direction = dst.get_turning_direction(&next_direction);
                    (true, Some(next_direction), turning_direction)
                } else {
                    (false, None, None)
                }
            }
            _ => unreachable!(),
        };

        r
    }

    fn get_inverted(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Ground,
    StartingPoint,
    Pipe(char),
    Inside,
}

impl Tile {
    fn is_pipe(&self) -> bool {
        matches!(self, Tile::Pipe(_))
    }

    fn display(&self) -> &str {
        match self {
            Tile::Ground => "•",
            Tile::StartingPoint => "S",
            Tile::Pipe(c) => match c {
                // feels a bit redundant
                '|' => "┃",
                '-' => "━",
                'L' => "┗",
                'J' => "┛",
                '7' => "┓",
                'F' => "┏",
                unknown => unreachable!("got unknown char: {}", unknown),
            },
            Tile::Inside => "X",
        }
    }

    fn get_direction_pair(&self) -> [Direction; 2] {
        match self {
            Tile::Pipe(c) => {
                match c {
                    // feels a bit redundant
                    '|' => [Direction::Up, Direction::Down],
                    '-' => [Direction::Left, Direction::Right],
                    'L' => [Direction::Up, Direction::Right],
                    'J' => [Direction::Up, Direction::Left],
                    '7' => [Direction::Left, Direction::Down],
                    'F' => [Direction::Right, Direction::Down],
                    unknown => unreachable!("got unknown char: {}", unknown),
                }
            }
            _ => unreachable!(),
        }
    }

    fn get_turning_direction(&self, next_direction: &Direction) -> Option<Direction> {
        match self {
            Tile::Pipe(c) => {
                let dir = match c {
                    'L' => {
                        if next_direction == &Direction::Up {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    }
                    'J' => {
                        if next_direction == &Direction::Up {
                            Direction::Left
                        } else {
                            Direction::Right
                        }
                    }
                    '7' => {
                        if next_direction == &Direction::Down {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    }
                    'F' => {
                        if next_direction == &Direction::Down {
                            Direction::Left
                        } else {
                            Direction::Right
                        }
                    }
                    _ => return None,
                };
                Some(dir)
            }
            _ => unreachable!(),
        }
    }

    fn from_vec(vec: [&Direction; 2]) -> Self {
        let c = match vec {
            // autofills arm
            [Direction::Up, Direction::Down] => '|',
            [Direction::Down, Direction::Up] => '|',
            [Direction::Left, Direction::Right] => '-',
            [Direction::Right, Direction::Left] => '-',
            [Direction::Up, Direction::Left] => 'J',
            [Direction::Left, Direction::Up] => 'J',
            [Direction::Right, Direction::Up] => 'L',
            [Direction::Up, Direction::Right] => 'L',
            [Direction::Down, Direction::Left] => '7',
            [Direction::Left, Direction::Down] => '7',
            [Direction::Down, Direction::Right] => 'F',
            [Direction::Right, Direction::Down] => 'F',
            _ => unreachable!(),
        };

        Self::Pipe(c)
    }

    fn get_floodfill_initial_coordinates(
        &self,
        current_direction: Direction,
        floodfill_side: Direction,
    ) -> (i32, i32) {
        assert!(floodfill_side == Direction::Left || floodfill_side == Direction::Right);

        match self {
            Tile::Pipe(c) => match c {
                '|' => match (floodfill_side, current_direction) {
                    (Direction::Left, Direction::Up) => (-1, 0),
                    (Direction::Left, Direction::Down) => (1, 0),
                    (Direction::Right, Direction::Up) => (1, 0),
                    (Direction::Right, Direction::Down) => (-1, 0),
                    _ => unreachable!(),
                },
                '-' => match (floodfill_side, current_direction) {
                    (Direction::Left, Direction::Left) => (-1, 0),
                    (Direction::Left, Direction::Right) => (1, 0),
                    (Direction::Right, Direction::Left) => (1, 0),
                    (Direction::Right, Direction::Right) => (-1, 0),
                    _ => unreachable!(),
                },
                'L' => match (floodfill_side, current_direction) {
                    (Direction::Left, Direction::Up) => (-1, 0),
                    (Direction::Left, Direction::Right) => (1, 0),
                    (Direction::Right, Direction::Up) => (1, 0),
                    (Direction::Right, Direction::Right) => (-1, 0),
                    _ => unreachable!(),
                },
                'J' => match (floodfill_side, current_direction) {
                    (Direction::Left, Direction::Up) => (-1, 0),
                    (Direction::Left, Direction::Left) => (0, -1),
                    (Direction::Right, Direction::Up) => (1, 0),
                    (Direction::Right, Direction::Left) => (0, 1),
                    _ => unreachable!(),
                },
                '7' => match (floodfill_side, current_direction) {
                    (Direction::Left, Direction::Down) => (1, 0),
                    (Direction::Left, Direction::Left) => (0, -1),
                    (Direction::Right, Direction::Down) => (-1, 0),
                    (Direction::Right, Direction::Left) => (0, 1),
                    _ => unreachable!(),
                },
                'F' => match (floodfill_side, current_direction) {
                    (Direction::Left, Direction::Down) => (1, 0),
                    (Direction::Left, Direction::Right) => (0, 1),
                    (Direction::Right, Direction::Down) => (-1, 0),
                    (Direction::Right, Direction::Right) => (0, -1),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Queue {
    coordinates: (i32, i32),
    direction: Direction,
    distance: i32,
}

impl Queue {
    fn new(coordinates: (i32, i32), direction: Direction, distance: i32) -> Self {
        Self {
            coordinates,
            direction,
            distance,
        }
    }

    fn get_next_coordinate(&self) -> (i32, i32) {
        let dir_val = self.direction.value();
        (
            self.coordinates.0 + dir_val.0,
            self.coordinates.1 + dir_val.1,
        )
    }
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<Tile>>,
    fence_map: Vec<Vec<Tile>>,
    starting_position: (i32, i32),
    starting_pipe_direction: Vec<Direction>,
    longest_starting_queue: Option<Queue>,
    turning_directions: Option<Direction>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut map = vec![];
        let mut fence_map = vec![];
        let mut starting_position = (i32::MAX, i32::MAX);

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            let mut normal_line_vec = vec![];
            let mut fence_line_vec = vec![];
            for c in line.chars() {
                let kind = match c {
                    '.' => Tile::Ground,
                    'S' => Tile::StartingPoint,
                    _ => Tile::Pipe(c),
                };
                normal_line_vec.push(kind);
                fence_line_vec.push(Tile::Ground);
            }

            map.push(normal_line_vec);
            fence_map.push(fence_line_vec);
        }

        // Reverse the Y-axis
        // Inputs are read from top to bottom with 0 at the top.
        // We reverse this so that 0 is at the bottom, aligning with the conventional coordinate system.
        map.reverse();

        // find index of starting position
        for (y, x_vec) in map.iter().enumerate() {
            for (x, val) in x_vec.iter().enumerate() {
                if val == &Tile::StartingPoint {
                    starting_position = (x as i32, y as i32);
                }
            }
        }

        assert!(starting_position.1 < map.len() as i32);

        Self {
            map,
            fence_map,
            starting_position,
            longest_starting_queue: None,
            starting_pipe_direction: vec![],
            turning_directions: None,
        }
    }

    fn display(&self, fence_view: bool) {
        let mut text = "\n".to_string();
        let mut map = match fence_view {
            true => self.fence_map.clone(),
            false => self.map.clone(),
        };

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

    fn get_tile(&self, coordinates: (i32, i32), fence_map: bool) -> Option<Tile> {
        let map = if fence_map {
            &self.fence_map
        } else {
            &self.map
        };

        let x = coordinates.0;
        let y = coordinates.1;

        if x < 0 || y < 0 || x >= map[0].len() as i32 || y >= map.len() as i32 {
            return None;
        }

        Some(map[y as usize][x as usize])
    }

    fn max_distance(&mut self) -> i32 {
        let mut walk_distance = i32::MIN;
        let mut longest_starting_queue = None;
        let mut turning_directions = None;

        for (coordinates_mod, possible_targets, direction) in [
            (
                (1, 0),
                [Tile::Pipe('-'), Tile::Pipe('7'), Tile::Pipe('J')],
                Direction::Right,
            ), // from starting point to right
            (
                (-1, 0),
                [Tile::Pipe('-'), Tile::Pipe('L'), Tile::Pipe('F')],
                Direction::Left,
            ), // from starting point to left
            (
                (0, 1),
                [Tile::Pipe('|'), Tile::Pipe('7'), Tile::Pipe('F')],
                Direction::Up,
            ), // from starting point to up
            (
                (0, -1),
                [Tile::Pipe('|'), Tile::Pipe('L'), Tile::Pipe('J')],
                Direction::Down,
            ), // from starting point to down
        ] {
            if let Some(next) = self.get_tile(
                (
                    self.starting_position.0 + coordinates_mod.0,
                    self.starting_position.1 + coordinates_mod.1,
                ),
                false,
            ) {
                if possible_targets.contains(&next) {
                    self.starting_pipe_direction.push(direction);
                    let initial_queue = Queue::new(self.starting_position, direction, 0);

                    let (next_walk_distance, local_turning_directions) =
                        self.walk(initial_queue.clone(), false, None);

                    if next_walk_distance > walk_distance {
                        walk_distance = next_walk_distance;
                        longest_starting_queue = Some(initial_queue.clone());

                        if !local_turning_directions.is_empty() {
                            turning_directions = local_turning_directions
                                .iter()
                                .max_by(|a, b| a.1.cmp(b.1))
                                .map(|(key, _)| *key);
                        }
                    }
                }
            }
        }

        self.longest_starting_queue = longest_starting_queue;

        assert!(turning_directions.is_some());
        self.turning_directions = turning_directions;

        num::Integer::div_ceil(&walk_distance, &2)
    }

    fn walk(
        &mut self,
        initial_queue: Queue,
        mark_fence: bool,
        floodfill_side: Option<Direction>,
    ) -> (i32, HashMap<Direction, i32>) {
        assert!(
            floodfill_side == Some(Direction::Left)
                || floodfill_side == Some(Direction::Right)
                || floodfill_side.is_none()
        );
        let mut walk_distance = i32::MIN;
        let mut queues = Vec::from([initial_queue]);
        let mut turning_directions = HashMap::new();

        while let Some(queue) = queues.pop() {
            let current_coordinates = queue.coordinates;
            let current_tile = self.get_tile(current_coordinates, false);
            if mark_fence {
                let tile = self.get_tile(current_coordinates, false).unwrap();
                self.fence_map[current_coordinates.1 as usize][current_coordinates.0 as usize] =
                    tile;
            }

            let next_coordinates = queue.get_next_coordinate();
            let next_tile = self.get_tile(next_coordinates, false);

            #[allow(clippy::unnecessary_unwrap)]
            if floodfill_side.is_some() && current_tile.is_some_and(|f| f.is_pipe()) {
                let side = floodfill_side.unwrap();
                let (x, y) = current_tile
                    .unwrap()
                    .get_floodfill_initial_coordinates(queue.direction, side);

                let mut stacks = vec![(queue.coordinates.0 + x, queue.coordinates.1 + y)];

                while let Some(coordinates) = stacks.pop() {
                    let current_tile = self.get_tile(coordinates, true);
                    if current_tile == Some(Tile::Ground) {
                        self.fence_map[coordinates.1 as usize][coordinates.0 as usize] =
                            Tile::Inside;
                        // this should be current coordinates +(1,0).... instead of (1,0)
                        for c in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            stacks.push((coordinates.0 + c.0, coordinates.1 + c.1))
                        }
                    }
                }
            }

            if next_tile.is_none() {
                continue;
            }

            let next_tile = next_tile.unwrap();

            if next_tile == Tile::StartingPoint {
                if queue.distance > walk_distance {
                    walk_distance = queue.distance;
                }
                walk_distance = std::cmp::max(walk_distance, queue.distance);
            }

            let (can_travel, next_direction, turning_direction) =
                queue.direction.can_travel_to(next_tile);

            if let Some(t) = turning_direction {
                *turning_directions.entry(t).or_insert(0) += 1;
            }

            if !can_travel {
                continue;
            }

            if next_direction.is_none() {
                continue;
            }

            let next_queue = Queue::new(
                next_coordinates,
                next_direction.unwrap(),
                queue.distance + 1,
            );

            queues.push(next_queue);
        }

        (walk_distance, turning_directions)
    }

    fn fill_fence_map(&mut self) -> i32 {
        assert!(self.longest_starting_queue.is_some());
        let mut inside_count = 0;

        // mark fence first
        self.walk(
            self.longest_starting_queue.as_ref().unwrap().clone(),
            true,
            None,
        );

        // and then floodfill
        self.walk(
            self.longest_starting_queue.as_ref().unwrap().clone(),
            false,
            self.turning_directions,
        );

        // replace starting point with actual pipe in fence map
        assert_eq!(self.starting_pipe_direction.len(), 2);
        self.fence_map[self.starting_position.1 as usize][self.starting_position.0 as usize] =
            Tile::from_vec([
                self.starting_pipe_direction.first().unwrap(),
                self.starting_pipe_direction.last().unwrap(),
            ]);

        for y_row in &self.fence_map {
            inside_count += y_row.iter().filter(|&x| x == &Tile::Inside).count() as i32;
        }

        inside_count
    }
}

pub fn solve(input: &str) -> Result<Answer> {
    let mut answer = Answer::default();

    let mut maze = Maze::new(input);
    maze.display(false);
    let part1 = maze.max_distance();
    let part2 = maze.fill_fence_map();
    maze.display(true);

    answer.part1 = Some(part1.to_string());
    answer.part2 = Some(part2.to_string());
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tracing_test::traced_test;

    use crate::day10::solve;

    #[traced_test]
    #[test]
    fn test_part1_1() -> Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let answer = solve(input)?;

        assert_eq!(answer.part1, Some("8".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part1_2() -> Result<()> {
        let input = ".....
.....
F---7
|---|
S---J";
        let answer = solve(input)?;

        assert_eq!(answer.part1, Some("6".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2_1() -> Result<()> {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let answer = solve(input)?;

        assert_eq!(answer.part2, Some("4".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2_2() -> Result<()> {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let answer = solve(input)?;

        assert_eq!(answer.part2, Some("8".to_string()));

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part2_3() -> Result<()> {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let answer = solve(input)?;

        assert_eq!(answer.part2, Some("10".to_string()));

        Ok(())
    }
}
