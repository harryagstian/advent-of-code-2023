use num::Integer;

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
