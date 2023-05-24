use std::marker::PhantomData;

use rostware23_lib::game::board::Board;
use rostware23_lib::game::board_bitset::Bitset8x8;
use rostware23_lib::game::state::State;
use rostware23_lib::game::penguin::{PenguinPossibleMoveIterator, Penguin};
use rostware23_lib::xml::common::Team;
use super::Rater;
use super::penguin_restrictions::*;

pub struct RestrictedReachableFishRater<Restrictions: PenguinRestrictions> {
    phantom: PhantomData<Restrictions>
}

impl<Restrictions: PenguinRestrictions> RestrictedReachableFishRater<Restrictions> {
    fn get_reachable_fish_from_penguin(penguin: Penguin, penguin_restrictions: &Restrictions, board: &Board, checked_field_bitset: &mut Bitset8x8) -> i32 {
        let penguin_move_iter = PenguinPossibleMoveIterator::from(penguin.clone(), board.clone());
        penguin_move_iter.fold(0, |accum, current_move| {
            let coordinate = current_move.get_to();
            let coordinate_in_bitset = coordinate.clone().doubled_to_odd_r();

            if checked_field_bitset.get(coordinate_in_bitset.x(), coordinate_in_bitset.y()).unwrap() {
                return accum;
            }
            checked_field_bitset.set(coordinate_in_bitset.x(), coordinate_in_bitset.y(), true).unwrap();

            if penguin_restrictions.is_restricted(coordinate.clone()) {
                return accum;
            }
            
            let field_state = board.get(coordinate.clone()).unwrap();
            let fish_count = field_state.get_fish_count();
            if let Ok(fish_count) = fish_count {
                let new_penguin = Penguin { coordinate, team: penguin.team.clone() };
                accum 
                    + fish_count as i32
                    + Self::get_reachable_fish_from_penguin(new_penguin, penguin_restrictions, board, checked_field_bitset) 
            } else {
                accum
            }
        })
    }

    fn reachable_fish_count_of_team(game_state: &State, team: Team) -> i32 {
        let mut total_reachable_fish = 0;
        let mut checked_field_bitset = Bitset8x8::empty();
        let penguins = game_state.board.get_penguin_iterator(team);
        for penguin in penguins {
            let penguin_restrictions = Restrictions::for_penguin(&penguin, &game_state.board);
            total_reachable_fish += Self::get_reachable_fish_from_penguin(penguin, &penguin_restrictions, &game_state.board, &mut checked_field_bitset);
        }
        total_reachable_fish
    }
}

impl<Restrictions: PenguinRestrictions> Rater for RestrictedReachableFishRater<Restrictions> {
    fn rate(game_state: &State) -> i32 {
        let current_team = game_state.current_team();
        Self::reachable_fish_count_of_team(game_state, current_team) 
            - Self::reachable_fish_count_of_team(game_state, current_team.opponent())
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::state::FieldState;

    use crate::logic::bitset_penguin_restrictions::BitsetPenguinRestrictions;
    use crate::logic::vec_penguin_restrictions::VecPenguinRestrictions;

    use super::*;

    #[test]
    fn given_empty_board_with_team_one_penguin_on_one_tile_island_then_reachable_fish_count_with_vec_restrictions_of_team_one_is_zero() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<VecPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(0, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_one_fish_then_reachable_fish_count_with_vec_restrictions_is_one() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<VecPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(1, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_fish_fields_around_then_reachable_fish_count_with_vec_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<VecPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(9, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_bigger_fish_field_island_then_reachable_fish_count_with_vec_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 2), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 3), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(6, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(9, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(11, 5), FieldState::Fish(3)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<VecPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(23, fish_count);
    }

    #[test]
    fn given_one_fish_filled_board_with_one_penguin_then_reachable_fish_count_with_vec_restrictions_is_63() {
        let mut board = Board::fill(FieldState::Fish(1));
        board.perform_move(Move::Place(Coordinate::new(1, 3)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<VecPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(63, fish_count);
    }

    #[test]
    fn given_board_with_two_penguins_and_bigger_fish_field_island_then_reachable_fish_count_with_vec_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 2), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 3), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(13, 3)), Team::One).unwrap();
        board.set(Coordinate::new(6, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(9, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(11, 5), FieldState::Fish(3)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<VecPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(23, fish_count);
    }

    #[test]
    fn given_board_with_restricted_part_then_reachable_fish_count_with_vec_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(6, 2), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(8, 2), FieldState::Fish(3)).unwrap();

        board.set(Coordinate::new(5, 3), FieldState::Fish(2)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(7, 3)), Team::Two).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(11, 3)), Team::One).unwrap();

        board.set(Coordinate::new(6, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(10, 4), FieldState::Fish(4)).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<VecPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(9, fish_count);
    }

    #[test]
    fn given_board_with_penguins_of_both_teams_and_bigger_fish_field_island_then_rating_with_vec_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 2), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 3), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(9, 3)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(13, 3)), Team::One).unwrap();
        board.set(Coordinate::new(6, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(10, 4), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(9, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(11, 5), FieldState::Fish(3)).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let actual = RestrictedReachableFishRater::<VecPenguinRestrictions>::rate(&state);
        assert_eq!(13 - 23, actual);
    }


    #[test]
    fn given_empty_board_with_team_one_penguin_on_one_tile_island_then_reachable_fish_count_with_bitset_restrictions_of_team_one_is_zero() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<BitsetPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(0, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_one_fish_then_reachable_fish_count_with_bitset_restrictions_is_one() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<BitsetPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(1, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_fish_fields_around_then_reachable_fish_count_with_bitset_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<BitsetPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(9, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_bigger_fish_field_island_then_reachable_fish_count_with_bitset_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 2), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 3), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(6, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(9, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(11, 5), FieldState::Fish(3)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<BitsetPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(23, fish_count);
    }

    #[test]
    fn given_one_fish_filled_board_with_one_penguin_then_reachable_fish_count_with_bitset_restrictions_is_63() {
        let mut board = Board::fill(FieldState::Fish(1));
        board.perform_move(Move::Place(Coordinate::new(1, 3)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<BitsetPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(63, fish_count);
    }

    #[test]
    fn given_board_with_two_penguins_and_bigger_fish_field_island_then_reachable_fish_count_with_bitset_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 2), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 3), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(13, 3)), Team::One).unwrap();
        board.set(Coordinate::new(6, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(9, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(11, 5), FieldState::Fish(3)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<BitsetPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(23, fish_count);
    }

    #[test]
    fn given_board_with_restricted_part_then_reachable_fish_count_with_bitset_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(6, 2), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(8, 2), FieldState::Fish(3)).unwrap();

        board.set(Coordinate::new(5, 3), FieldState::Fish(2)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(7, 3)), Team::Two).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(11, 3)), Team::One).unwrap();

        board.set(Coordinate::new(6, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(10, 4), FieldState::Fish(4)).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = RestrictedReachableFishRater::<BitsetPenguinRestrictions>::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(9, fish_count);
    }

    #[test]
    fn given_board_with_penguins_of_both_teams_and_bigger_fish_field_island_then_rating_with_bitset_restrictions_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 2), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 3), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(9, 3)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(13, 3)), Team::One).unwrap();
        board.set(Coordinate::new(6, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(10, 4), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(7, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(9, 5), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(11, 5), FieldState::Fish(3)).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let actual = RestrictedReachableFishRater::<BitsetPenguinRestrictions>::rate(&state);
        assert_eq!(13 - 23, actual);
    }
}
