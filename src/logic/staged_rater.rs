use std::marker::PhantomData;

use rostware23_lib::game::state::State;

use super::Rater;

const EARLY_GAME_MAX_TURN: u32 = 8;

pub struct StagedRater<
    const EARLY_MULTIPLIER: i32,
    const MID_MULTIPLIER: i32,
    const END_MULTIPLIER: i32,
    Heuristic: Rater,
> {
    phantom: PhantomData<Heuristic>,
}

impl<
        const EARLY_MULTIPLIER: i32,
        const MID_MULTIPLIER: i32,
        const END_MULTIPLIER: i32,
        Heuristic: Rater,
    > Rater for StagedRater<EARLY_MULTIPLIER, MID_MULTIPLIER, END_MULTIPLIER, Heuristic>
{
    fn rate(state: &State) -> i32 {
        if state.turn < EARLY_GAME_MAX_TURN {
            return EARLY_MULTIPLIER * Heuristic::rate(state);
        }

        let opponent_team = state.current_team().opponent();
        if !state.has_team_any_moves(opponent_team) {
            return END_MULTIPLIER * Heuristic::rate(state);
        }

        MID_MULTIPLIER * Heuristic::rate(state)
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::board::Board;
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::common::Team;
    use rostware23_lib::xml::state::FieldState;

    use super::*;

    struct TestRater;

    impl Rater for TestRater {
        fn rate(_: &State) -> i32 {
            1
        }
    }

    type TestStagedRater = StagedRater::<2, 16, 91, TestRater>;

    #[test]
    fn staged_rater_in_early_game_with_test_rater_returns_expected_rating() {
        let board = Board::empty();
        let state = State::from_initial_board_with_start_team_one(board);
        assert_eq!(2, TestStagedRater::rate(&state));
    }

    #[test]
    fn staged_rater_in_mid_game_with_test_rater_returns_expected_rating() {
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
            .perform_move(Move::Place(Coordinate::new(1, 1)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(3, 1)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(5, 1)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(7, 1)), Team::Two)
            .unwrap();
        let mut state = State::from_initial_board_with_start_team_one(board);
        state.turn = 8;
        assert_eq!(16, TestStagedRater::rate(&state));
    }

    #[test]
    fn staged_rater_in_end_game_with_test_rater_returns_expected_rating() {
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
            .perform_move(Move::Place(Coordinate::new(2, 4)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(4, 4)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(6, 4)), Team::Two)
            .unwrap();
        board
            .perform_move(Move::Place(Coordinate::new(8, 4)), Team::Two)
            .unwrap();
        let mut state = State::from_initial_board_with_start_team_one(board);
        state.turn = 26;
        assert_eq!(91, TestStagedRater::rate(&state));
    }
}
