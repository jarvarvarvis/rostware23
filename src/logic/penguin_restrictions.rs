use rostware23_lib::game::common::{Coordinate, Vector};
use rostware23_lib::game::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PenguinRestriction {
    restricting_penguin_coords: Coordinate,
    direction_to_start_penguin: Direction
}

impl PenguinRestriction {
    pub fn new(restricting_penguin_coords: Coordinate, direction_to_start_penguin: Direction) -> Self { 
        Self { 
            restricting_penguin_coords, 
            direction_to_start_penguin 
        } 
    }

    fn highest_reachable_angle() -> f64 {
        let left = Direction::Left.vector();
        let top_right = Direction::TopRight.vector();
        left.angle_to(top_right)
    }

    pub fn is_in_restriction(&self, coordinate: Coordinate) -> bool {
        let vector = Vector::between_coordinates(coordinate, self.restricting_penguin_coords.clone());
        let direction_vector = self.direction_to_start_penguin.vector();
        let angle = direction_vector.angle_to(vector);
        angle >= Self::highest_reachable_angle()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_towards_top_right_is_in_restriction_to_the_left() {
        let restriction = PenguinRestriction::new(Coordinate::new(10, 6), Direction::Left);
        let checked_coordinate = Coordinate::new(11, 5);
        assert!(restriction.is_in_restriction(checked_coordinate));
    }

    #[test]
    fn coordinate_towards_top_right_far_from_origin_is_in_restriction_to_the_left() {
        let restriction = PenguinRestriction::new(Coordinate::new(10, 6), Direction::Left);
        let checked_coordinate = Coordinate::new(15, 7);
        assert!(restriction.is_in_restriction(checked_coordinate));
    }

    #[test]
    fn coordinate_towards_top_left_is_not_in_restriction_to_the_left() {
        let restriction = PenguinRestriction::new(Coordinate::new(10, 6), Direction::Left);
        let checked_coordinate = Coordinate::new(9, 7);
        assert!(!restriction.is_in_restriction(checked_coordinate));
    }

    #[test]
    fn coordinate_towards_top_left_far_from_origin_is_not_in_restriction_to_the_left() {
        let restriction = PenguinRestriction::new(Coordinate::new(10, 6), Direction::Left);
        let checked_coordinate = Coordinate::new(3, 6);
        assert!(!restriction.is_in_restriction(checked_coordinate));
    }
}
