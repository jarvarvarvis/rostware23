use rostware23_lib::game::board::Board;
use rostware23_lib::game::common::{Coordinate, Vector};
use rostware23_lib::game::direction::Direction;
use rostware23_lib::game::penguin::Penguin;

#[derive(Debug, PartialEq)]
pub struct PenguinRestriction {
    restricting_penguin_coords: Coordinate,
    direction_to_start_penguin: Direction,
    highest_reachable_angle: f64
}

impl PenguinRestriction {
    pub fn new(restricting_penguin_coords: Coordinate, direction_to_start_penguin: Direction) -> Self { 
        let left = Direction::Left.vector();
        let top_right = Direction::TopRight.vector();
        Self { 
            restricting_penguin_coords, 
            direction_to_start_penguin,
            highest_reachable_angle: left.angle_to(top_right)
        } 
    }

    pub fn is_in_restriction(&self, coordinate: Coordinate) -> bool {
        let vector = Vector::between_coordinates(coordinate, self.restricting_penguin_coords.clone());
        let direction_vector = self.direction_to_start_penguin.vector();
        let angle = direction_vector.angle_to(vector);
        angle >= self.highest_reachable_angle
    }
}

pub trait PenguinRestrictions {
    fn for_penguin(penguin: &Penguin, board: &Board) -> Self;
    fn is_restricted(&self, coordinate: Coordinate) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highest_reachable_angle_is_correct() {
        let actual = PenguinRestriction::new(Coordinate::new(0, 0), Direction::Left).highest_reachable_angle;
        let degrees: f64 = 135.0;
        assert_eq!(degrees.to_radians(), actual); 
    }

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
}

