use rostware23_lib::game::board::Board;
use rostware23_lib::game::board_bitset::Bitset8x8;
use rostware23_lib::game::state::State;
use rostware23_lib::game::penguin::{PenguinPossibleMoveIterator, Penguin};
use rostware23_lib::xml::common::Team;
use super::Rater;

pub struct ReachableFishRater;

impl ReachableFishRater {
    fn get_reachable_fish_from_penguin(penguin: Penguin, board: &Board, checked_field_bitset: &mut Bitset8x8) -> i32 {
        let penguin_move_iter = PenguinPossibleMoveIterator::from(penguin.clone(), board.clone());
        penguin_move_iter.fold(0, |accum, current_move| {
            let coordinate = current_move.get_to();
            let coordinate_in_bitset = coordinate.clone().doubled_to_odd_r();

            if checked_field_bitset.get(coordinate_in_bitset.x(), coordinate_in_bitset.y()).unwrap_or(false) {
                return accum;
            }
            checked_field_bitset.set(coordinate_in_bitset.x(), coordinate_in_bitset.y(), true).unwrap();
            
            let field_state = board.get(coordinate.clone()).unwrap();
            let fish_count = field_state.get_fish_count();
            if let Ok(fish_count) = fish_count {
                let new_penguin = Penguin { coordinate, team: penguin.team.clone() };
                accum + Self::get_reachable_fish_from_penguin(new_penguin, board, checked_field_bitset) + fish_count as i32
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
            let fish_count = Self::get_reachable_fish_from_penguin(penguin, &game_state.board, &mut checked_field_bitset);
            total_reachable_fish += fish_count;
        }
        total_reachable_fish
    }
}

impl Rater for ReachableFishRater {
    fn rate(game_state: &State) -> i32 {
        let current_team = game_state.current_team();
        Self::reachable_fish_count_of_team(game_state, current_team) - Self::reachable_fish_count_of_team(game_state, current_team.clone())
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::state::FieldState;

    use super::*;

    #[test]
    fn given_empty_board_with_team_one_penguin_on_one_tile_island_then_reachable_fish_count_of_team_one_is_zero() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = ReachableFishRater::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(0, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_one_fish_then_reachable_fish_count_is_one() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = ReachableFishRater::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(1, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_fish_fields_around_then_reachable_fish_count_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(8, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(12, 4), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(9, 3), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(11, 3), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(10, 4)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = ReachableFishRater::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(9, fish_count);
    }

    #[test]
    fn given_board_with_one_penguin_and_bigger_fish_field_island_then_reachable_fish_count_is_correct() {
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
        let fish_count = ReachableFishRater::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(23, fish_count);
    }

    #[test]
    fn given_one_fish_filled_board_with_one_penguin_then_reachable_fish_count_is_63() {
        let mut board = Board::fill(FieldState::Fish(1));
        board.perform_move(Move::Place(Coordinate::new(1, 3)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let fish_count = ReachableFishRater::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(63, fish_count);
    }

    #[test]
    fn given_board_with_two_penguin_and_bigger_fish_field_island_then_reachable_fish_count_is_correct() {
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
        let fish_count = ReachableFishRater::reachable_fish_count_of_team(&state, Team::One);
        assert_eq!(23, fish_count);
    }
}
