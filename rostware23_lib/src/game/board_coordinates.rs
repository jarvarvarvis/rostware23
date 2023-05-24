use super::common::*;

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
