use std::str::FromStr;

use num::Integer;
use strum::EnumIter;

#[derive(PartialEq)]
pub enum Part {
    One,
    Two,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn is_vertical(&self) -> bool {
        match self {
            Direction::East | Direction::West | Direction::Left | Direction::Right => false,
            Direction::South | Direction::North | Direction::Up | Direction::Down => true,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::East | Direction::West | Direction::Left | Direction::Right => true,
            Direction::South | Direction::North | Direction::Up | Direction::Down => false,
        }
    }

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    pub fn get_modifier(&self, increment: i32) -> (i32, i32) {
        match self {
            Direction::North | Direction::Up => (0, increment),
            Direction::East | Direction::Left => (-increment, 0),
            Direction::South | Direction::Down => (0, -increment),
            Direction::West | Direction::Right => (increment, 0),
        }
    }

    pub fn display(&self) -> &str {
        match self {
            Direction::North | Direction::Up => "↑",
            Direction::East | Direction::Left => "←",
            Direction::South | Direction::Down => "↓",
            Direction::West | Direction::Right => "→",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            _ => return Err(ParseDirectionError),
        };

        Ok(result)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: Integer + Copy> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn add(&self, x: T, y: T) -> Self {
        let new_x = self.x + x;
        let new_y = self.y + y;
        Self::new(new_x, new_y)
    }
}

pub fn get_column<T: Copy>(slice: &[Vec<T>], index: i32) -> Option<Vec<T>> {
    assert!(!slice.is_empty());
    let len = slice[0].len();
    if index < 0 || index as usize >= len {
        None
    } else {
        assert!(slice.iter().all(|f| f.len() == len));
        Some(slice.iter().map(|row| row[index as usize]).collect())
    }
}

pub fn get_row<T: Copy>(slice: &[Vec<T>], index: i32) -> Option<Vec<T>> {
    assert!(!slice.is_empty());
    let len = slice.len();
    if index < 0 || index as usize >= len {
        None
    } else {
        Some(slice[index as usize].clone())
    }
}

pub fn update_column<T: Copy>(
    map: &mut [Vec<T>],
    new: &[T],
    column_index: i32,
    should_reverse: bool,
) {
    assert!(!new.is_empty());
    assert_eq!(new.len(), map.len());
    let mut new = new.to_vec();

    if should_reverse {
        new.reverse();
    }

    for (row_index, value) in new.iter().enumerate() {
        map[row_index][column_index as usize] = *value
    }
}

pub fn update_row<T: Copy>(map: &mut [Vec<T>], new: &[T], row_index: i32, should_reverse: bool) {
    assert!(!new.is_empty());
    assert_eq!(new.len(), map[row_index as usize].len());
    let mut new = new.to_vec();

    if should_reverse {
        new.reverse();
    }

    map[row_index as usize] = new;
}
