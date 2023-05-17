pub use crate::xml::common::*;

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

    pub fn odd_r_to_doubled(self) -> Self {
        let x = self.x();
        let y = self.y();
        Self(x * 2 + y % 2, y)
    }

    pub fn doubled_to_odd_r(self) -> Self {
        let x = self.x();
        let y = self.y();
        Self((x as f32 / 2.0).ceil() as u64 - y % 2, y)
    }
}
