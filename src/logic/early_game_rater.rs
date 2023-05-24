use std::marker::PhantomData;

use rostware23_lib::{game::state::State, xml::common::Team};

use super::Rater;

const TOTAL_MAX_PENGUIN_COUNT: usize = 8;

pub struct EarlyGameRater<Heuristic: Rater> {
    phantom: PhantomData<Heuristic>
}

impl<Heuristic: Rater> Rater for EarlyGameRater<Heuristic> {
    fn rate(state: &State) -> i32 {
        let team_one_penguins = state.board.get_penguin_iterator(Team::One);
        let team_two_penguins = state.board.get_penguin_iterator(Team::Two);
        let penguin_count = team_one_penguins.count() + team_two_penguins.count();
        if penguin_count < TOTAL_MAX_PENGUIN_COUNT {
            Heuristic::rate(state)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::board::Board;
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::state::FieldState;

    use super::*;

    struct TestRater;

    const TEST_RATER_VALUE: i32 = 123;

    impl Rater for TestRater {
        fn rate(_: &State) -> i32 {
            TEST_RATER_VALUE
        }
    }

    #[test]
    fn early_game_rater_returns_test_rater_value_when_in_early_game() {
        let board = Board::empty();
        let state = State::from_initial_board_with_start_team_one(board);
        let rating = EarlyGameRater::<TestRater>::rate(&state);
        assert_eq!(TEST_RATER_VALUE, rating);
    }

    #[test]
    fn early_game_rater_returns_zero_when_not_in_early_game_on_simple_board() {
        let mut board = Board::empty();
        board
            .set(Coordinate::new(0, 0), FieldState::Fish(2))
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(2, 0)), Team::One)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(4, 0)), Team::One)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(6, 0)), Team::One)
            .unwrap();
        board
            .set(Coordinate::new(8, 0), FieldState::Fish(2))
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(12, 0)), Team::One)
            .unwrap();

        board
            .perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(4, 2)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(6, 2)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(8, 2)), Team::Two)
            .unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let rating = EarlyGameRater::<TestRater>::rate(&state);
        assert_eq!(0, rating);
    }
}
