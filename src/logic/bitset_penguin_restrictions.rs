use rostware23_lib::game::board::Board;
use rostware23_lib::game::board_bitset::Bitset8x8;
use rostware23_lib::game::common::{Coordinate, Vector, BOARD_HEIGHT, BOARD_WIDTH};
use rostware23_lib::game::direction::DirectionIterator;
use rostware23_lib::game::penguin::Penguin;

use super::penguin_restrictions::*;

#[derive(Debug, PartialEq)]
pub struct BitsetPenguinRestrictions {
    restriction_bitset: Bitset8x8,
}

impl BitsetPenguinRestrictions {
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

    fn from_restrictions(restrictions: Vec<PenguinRestriction>) -> Self {
        let mut restriction_bitset = Bitset8x8::empty();
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                for restriction in restrictions.iter() {
                    if restriction.is_in_restriction(Coordinate::new(x, y).odd_r_to_doubled()) {
                        restriction_bitset.set(x, y, true).unwrap();
                    }
                }
            }
        }
        Self { restriction_bitset }
    }
}

impl PenguinRestrictions for BitsetPenguinRestrictions {
    fn for_penguin(penguin: &Penguin, board: &Board) -> Self {
        let opponent_penguins = board.get_penguin_iterator(penguin.team.opponent());
        let restrictions = opponent_penguins
            .filter_map(|opponent_penguin| {
                let vector_to_opponent = Vector::between_coordinates(opponent_penguin.coordinate.clone(), 
                                                                     penguin.coordinate.clone());
                Self::find_restriction_for_opponent_penguin(opponent_penguin, vector_to_opponent)
            })
            .collect();
        Self::from_restrictions(restrictions)
    }

    fn is_restricted(&self, coordinate: Coordinate) -> bool {
        let odd_r_coordinate = coordinate.doubled_to_odd_r();
        self.restriction_bitset.get(odd_r_coordinate.x(), odd_r_coordinate.y()).unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::state::FieldState;
    use rostware23_lib::xml::common::Team;

    use super::*;

    #[test]
    fn penguin_is_not_restricted_towards_right_on_simple_board_using_bitset_restrictions() {
        let mut board = Board::empty();
        let penguin = Penguin { coordinate: Coordinate::new(0, 0), team: Team::One };
        board.perform_move(Move::Place(penguin.coordinate.clone()), penguin.team.clone()).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(1)).unwrap();
        let other_coord = Coordinate::new(4, 0);
        board.set(other_coord.clone(), FieldState::Fish(1)).unwrap();
        let restriction = BitsetPenguinRestrictions::for_penguin(&penguin, &board);
        assert!(!restriction.is_restricted(other_coord));
    }

    #[test]
    fn penguin_is_restricted_by_other_penguin_towards_right_on_simple_board_using_bitset_restrictions() {
        let mut board = Board::empty();
        let penguin = Penguin { coordinate: Coordinate::new(0, 0), team: Team::One };
        board.perform_move(Move::Place(penguin.coordinate.clone()), penguin.team.clone()).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::Two).unwrap();
        let other_coord = Coordinate::new(6, 0);
        board.set(other_coord.clone(), FieldState::Fish(1)).unwrap();
        let restriction = BitsetPenguinRestrictions::for_penguin(&penguin, &board);
        assert!(restriction.is_restricted(other_coord));
    }
}

