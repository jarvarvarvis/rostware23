use rostware23_lib::game::board::Board;
use rostware23_lib::game::common::{Coordinate, Vector};
use rostware23_lib::game::direction::DirectionIterator;
use rostware23_lib::game::penguin::Penguin;

use super::penguin_restrictions::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VecPenguinRestrictions {
    restrictions: Vec<PenguinRestriction>
}

impl VecPenguinRestrictions {
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
}

impl PenguinRestrictions for VecPenguinRestrictions {
    fn for_penguin(penguin: &Penguin, board: &Board) -> Self {
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

    fn is_restricted(&self, coordinate: Coordinate) -> bool {
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
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::state::FieldState;
    use rostware23_lib::xml::common::Team;

    use super::*;

    #[test]
    fn penguin_is_not_restricted_towards_right_on_simple_board_using_simple_restrictions() {
        let mut board = Board::empty();
        let penguin = Penguin { coordinate: Coordinate::new(0, 0), team: Team::One };
        board.perform_move(Move::Place(penguin.coordinate.clone()), penguin.team.clone()).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(1)).unwrap();
        let other_coord = Coordinate::new(4, 0);
        board.set(other_coord.clone(), FieldState::Fish(1)).unwrap();
        let restriction = VecPenguinRestrictions::for_penguin(&penguin, &board);
        assert!(!restriction.is_restricted(other_coord));
    }

    #[test]
    fn penguin_is_restricted_by_other_penguin_towards_right_on_simple_board_using_simple_restrictions() {
        let mut board = Board::empty();
        let penguin = Penguin { coordinate: Coordinate::new(0, 0), team: Team::One };
        board.perform_move(Move::Place(penguin.coordinate.clone()), penguin.team.clone()).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::Two).unwrap();
        let other_coord = Coordinate::new(6, 0);
        board.set(other_coord.clone(), FieldState::Fish(1)).unwrap();
        let restriction = VecPenguinRestrictions::for_penguin(&penguin, &board);
        assert!(restriction.is_restricted(other_coord));
    }
}
