use rostware23_lib::game::state::State;
use rostware23_lib::game::board::Board;
use rostware23_lib::game::moves::Move;
use rostware23_lib::xml::common::Team;
use rostware23_lib::game::possible_moves::PossibleMovesIterator;
use super::Rater;

pub struct PotentialFishRater {}

impl Rater for PotentialFishRater {
    fn rate(state: &State) -> i32 {
        let mut result: i32 = 0;
        let own_possible_moves = PossibleMovesIterator::make_normal_moves_iterator_for_team(state.clone(), state.current_team());
        for current_move in own_possible_moves {
            let target_fish = state.board.get(current_move.get_to()).unwrap().get_fish_count().unwrap() as i32;
            result += target_fish;
        }
        let opponent_possible_moves = PossibleMovesIterator::make_normal_moves_iterator_for_team(state.clone(), state.current_team().opponent());
        for current_move in opponent_possible_moves {
            let target_fish = state.board.get(current_move.get_to()).unwrap().get_fish_count().unwrap() as i32;
            result -= target_fish;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::board::*;

    #[test]
    fn given_empty_game_state_when_calculating_rating_returns_zero() {
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        assert_eq!(0, PotentialFishRater::rate(&state));
    }

    #[test]
    fn given_game_state_with_five_potential_fish_for_team_one_when_calculating_rating_returns_five() {
        let mut board = Board::empty();
        for i in 1..5 {
            board.perform_move(Move::Place(Coordinate::new(i*2, 0)), Team::One).unwrap();
            board.perform_move(Move::Place(Coordinate::new(i*2, 4)), Team::Two).unwrap();
        }
        board.set(Coordinate::new(10, 0), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(12, 0), FieldState::Fish(3)).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        assert_eq!(5, PotentialFishRater::rate(&state));
    }

    #[test]
    fn given_game_state_with_four_potential_fish_for_team_two_and_three_for_team_one_when_calculating_rating_for_team_two_returns_one() {
        let mut board = Board::empty();
        for i in 1..5 {
            board.perform_move(Move::Place(Coordinate::new(i*2, 0)), Team::One).unwrap();
            board.perform_move(Move::Place(Coordinate::new(i*2, 4)), Team::Two).unwrap();
        }
        board.set(Coordinate::new(10, 0), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(12, 0), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(10, 4), FieldState::Fish(4)).unwrap();
        let mut state = State::from_initial_board_with_start_team_one(board);
        state = state.with_move_performed(Move::Normal{from: Coordinate::new(8,0), to: Coordinate::new(10,0)}).unwrap();
        assert_eq!(1, PotentialFishRater::rate(&state));
    }

    #[test]
    fn uses_possible_move_rater_in_normal_case() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(0, 4)), Team::Two).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(2, 4), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(2, 2), FieldState::Fish(1)).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        assert_eq!(1, PotentialFishRater::rate(&state));
    }
}
