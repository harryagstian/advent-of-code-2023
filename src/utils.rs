use num::Integer;

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn is_vertical(&self) -> bool {
        match self {
            Direction::East | Direction::West => false,
            Direction::South | Direction::North => true,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::East | Direction::West => true,
            Direction::South | Direction::North => false,
        }
    }
}

pub struct Coordinate<T> {
    x: T,
    y: T,
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
