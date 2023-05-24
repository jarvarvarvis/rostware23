use rostware23_lib::game::board::Board;
use rostware23_lib::game::common::{Coordinate, Vector};
use rostware23_lib::game::direction::{Direction, DirectionIterator};
use rostware23_lib::game::penguin::Penguin;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct PenguinRestrictions {
    restrictions: Vec<PenguinRestriction>
}

impl PenguinRestrictions {
    fn find_restriction_for_opponent_penguin(opponent_penguin: Penguin, vector_to_opponent: Vector) -> Option<PenguinRestriction> {
        let vector_to_opponent_inverse = vector_to_opponent.scale(-1);
        for direction in DirectionIterator::new() {
            let direction_vector = direction.vector();
            if vector_to_opponent_inverse.clone().angle_to(direction_vector) == 0.0 {
                return Some(PenguinRestriction::new(
                    opponent_penguin.coordinate,
                    direction
                ));
            }
        }
        None
    }

    pub fn for_penguin(penguin: &Penguin, board: &Board) -> Self {
        let opponent_penguins = board.get_penguin_iterator(penguin.team.opponent());
        let restrictions = opponent_penguins
            .filter_map(|opponent_penguin| {
                let vector_to_opponent = Vector::between_coordinates(opponent_penguin.coordinate.clone(), 
                                                                     penguin.coordinate.clone());
                Self::find_restriction_for_opponent_penguin(opponent_penguin, vector_to_opponent)
            })
            .collect();
        Self {
            restrictions
        }
    }

    pub fn is_restricted(&self, coordinate: Coordinate) -> bool {
        for restriction in self.restrictions.iter() {
            if restriction.is_in_restriction(coordinate.clone()) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::xml::state::FieldState;
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::common::Team;

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

    #[test]
    fn penguin_is_not_restricted_towards_right_on_simple_board() {
        let mut board = Board::empty();
        let penguin = Penguin { coordinate: Coordinate::new(0, 0), team: Team::One };
        board.perform_move(Move::Place(penguin.coordinate.clone()), penguin.team.clone()).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(1)).unwrap();
        let other_coord = Coordinate::new(4, 0);
        board.set(other_coord.clone(), FieldState::Fish(1)).unwrap();
        let restriction = PenguinRestrictions::for_penguin(&penguin, &board);
        assert!(!restriction.is_restricted(other_coord));
    }

    #[test]
    fn penguin_is_restricted_by_other_penguin_towards_right_on_simple_board() {
        let mut board = Board::empty();
        let penguin = Penguin { coordinate: Coordinate::new(0, 0), team: Team::One };
        board.perform_move(Move::Place(penguin.coordinate.clone()), penguin.team.clone()).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::Two).unwrap();
        let other_coord = Coordinate::new(6, 0);
        board.set(other_coord.clone(), FieldState::Fish(1)).unwrap();
        let restriction = PenguinRestrictions::for_penguin(&penguin, &board);
        assert!(restriction.is_restricted(other_coord));
    }
}
