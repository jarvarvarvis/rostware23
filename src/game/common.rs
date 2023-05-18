pub use crate::xml::common::*;

pub const BOARD_WIDTH: u64 = 8;
pub const BOARD_HEIGHT: u64 = 8;

pub const RIGHTMOST_X: u64 = BOARD_WIDTH * 2 - 1;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector(i64, i64);

impl Vector {
    pub fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Coordinate(u64, u64);

impl Coordinate {
    pub fn new(x: u64, y: u64) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> u64 {
        self.0
    }

    pub fn y(&self) -> u64 {
        self.1
    }

    pub fn add(&self, vector: Vector) -> Self {
        Self::new((self.x() as i64 + vector.x()) as u64, 
                  (self.y() as i64 + vector.y()) as u64)
    }

    pub fn odd_r_to_doubled(self) -> Self {
        let x = self.x();
        let y = self.y();
        Self(x * 2 + y % 2, y)
    }

    pub fn is_valid(&self) -> bool {
        let x = self.x();
        let y = self.y();
        x <= RIGHTMOST_X && y < BOARD_HEIGHT && x % 2 == y % 2
    }
}

pub struct BoardCoordinateIterator {
    last: Coordinate
}

impl BoardCoordinateIterator {
    pub fn new() -> Self {
        Self { last: Coordinate::new(0, 0) }
    }
}

impl Iterator for BoardCoordinateIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last.is_valid() {
            let previous_coordinate = self.last.clone();
            let next_x_in_line = self.last.x() + 2;
            let next_y = self.last.y() + 1;
            let x_overflow = next_x_in_line > RIGHTMOST_X;
            let real_next_x = if x_overflow { next_y % 2 } else { next_x_in_line };
            let real_next_y = if x_overflow { next_y } else { self.last.y() };
            self.last = Coordinate::new(real_next_x, real_next_y);
            return Some(previous_coordinate);
        }
        None
    }
}
