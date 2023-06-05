use std::marker::PhantomData;

use rostware23_lib::game::board::Board;
use rostware23_lib::game::board_bitset::Bitset8x8;
use rostware23_lib::game::direction::DirectionIterator;
use rostware23_lib::game::state::State;
use rostware23_lib::game::penguin::Penguin;
use rostware23_lib::xml::common::Team;
use super::Rater;
use super::penguin_restrictions::*;

pub struct RestrictedReachableFishRater<Restrictions: PenguinRestrictions> {
    phantom: PhantomData<Restrictions>
}

impl<Restrictions: PenguinRestrictions> RestrictedReachableFishRater<Restrictions> {
    fn get_reachable_fish_from_penguin(penguin: Penguin, penguin_restrictions: &Restrictions, board: &Board, checked_field_bitset: &mut Bitset8x8) -> i32 {
        let direction_iter = DirectionIterator::new();
        let coord = penguin.coordinate;
        let coord8x8 = coord.clone().doubled_to_odd_r();
        let half_x = coord8x8.x();
        let y = coord8x8.y();
        if checked_field_bitset.get(half_x, y).unwrap() {
            return 0;
        }
        if penguin_restrictions.is_restricted(coord.clone()) {
            return 0;
        }
        let mut result = 0;
        checked_field_bitset.set(half_x, y, true).unwrap();
        match board.get(coord.clone()).unwrap().get_fish_count() {
            Ok(fish) => result += fish as i32,
            Err(_) => {}
        }
        for direction in direction_iter {
            let next_coord = coord.clone().add(direction.vector());
            if !next_coord.is_valid() {
                continue;
            }
            let next_fish = match board.get(next_coord.clone()).unwrap().get_fish_count() {
                Ok(fish) => fish,
                Err(_) => 0
            };
            if next_fish == 0 {
                continue;
            }
            let next_penguin = Penguin { coordinate: next_coord, team: penguin.team };
            result += Self::get_reachable_fish_from_penguin(next_penguin, penguin_restrictions, board, checked_field_bitset);
        }
        result
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
    use crate::logic::board_parser::parse_board;
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

    #[test]
    fn given_rather_complex_board_when_rating_for_both_teams_then_returns_correct_value_zero() {
        let board_string =
            "4 3 3 =   = = 3\n\
         - = P = = G - =\n\
        - = G = - - P -\n\
         = =     = - = -\n\
        - = - =     = =\n\
         - G - - = P = -\n\
        = - - = = - = -\n\
         3 = =   = 3 3 4\n";
        let board = parse_board(board_string);
        let game_state = State::from_initial_board_with_start_team_one(board);
        assert_eq!(0, RestrictedReachableFishRater::<BitsetPenguinRestrictions>::rate(&game_state));
    }
}
