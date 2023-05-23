use super::common::*;

pub const HALF_TILE_X_CHANGE: i64 = 1;
pub const FULL_TILE_X_CHANGE: i64 = 2;
pub const FULL_TILE_Y_CHANGE: i64 = 1;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    TopLeft,
    TopRight,
    Right,
    BottomRight,
    BottomLeft
}

impl Direction {
    #[inline] pub fn vector(&self) -> Vector {
        match self {
            Self::Left => Vector::new(-FULL_TILE_X_CHANGE, 0),
            Self::TopLeft => Vector::new(-HALF_TILE_X_CHANGE, FULL_TILE_Y_CHANGE),
            Self::TopRight => Vector::new(HALF_TILE_X_CHANGE, FULL_TILE_Y_CHANGE),
            Self::Right => Vector::new(FULL_TILE_X_CHANGE, 0),
            Self::BottomRight => Vector::new(HALF_TILE_X_CHANGE, -FULL_TILE_Y_CHANGE),
            Self::BottomLeft => Vector::new(-HALF_TILE_X_CHANGE, -FULL_TILE_Y_CHANGE)
        }
    }
}

pub struct DirectionIterator {
    current_direction: Option<Direction>
}

impl DirectionIterator {
    #[inline] pub fn new() -> Self {
        Self { current_direction: Some(Direction::Left) }
    }

    fn next_direction(direction: &Direction) -> Option<Direction> {
        match direction {
            Direction::Left => Some(Direction::TopLeft),
            Direction::TopLeft => Some(Direction::TopRight),
            Direction::TopRight => Some(Direction::Right),
            Direction::Right => Some(Direction::BottomRight),
            Direction::BottomRight => Some(Direction::BottomLeft),
            Direction::BottomLeft => None,
        }
    }
}

impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current_direction {
            Some(direction) => {
                let current_direction = direction.clone();
                self.current_direction = Self::next_direction(direction);
                Some(current_direction)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_iterator_yields_directions_in_order() {
        let mut direction_iterator = DirectionIterator::new();

        assert_eq!(Direction::Left, direction_iterator.next().unwrap());
        assert_eq!(Direction::TopLeft, direction_iterator.next().unwrap());
        assert_eq!(Direction::TopRight, direction_iterator.next().unwrap());
        assert_eq!(Direction::Right, direction_iterator.next().unwrap());
        assert_eq!(Direction::BottomRight, direction_iterator.next().unwrap());
        assert_eq!(Direction::BottomLeft, direction_iterator.next().unwrap());

        assert_eq!(None, direction_iterator.next());
    }
}
