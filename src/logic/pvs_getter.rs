use std::marker::PhantomData;

use anyhow::Context;
use rostware23_lib::game::common::{BOARD_WIDTH, BOARD_HEIGHT};
use rostware23_lib::game::moves::Move;
use rostware23_lib::game::state::State;

use super::MoveGetter;
use super::fish_difference_rater::FishDifferenceRater;
use super::ordered_move_generator::OrderedMoveGenerator;
use super::time_measurer::TimeMeasurer;
use super::Rater;

const INITIAL_LOWER_BOUND: i32 = -1000000;
const INITIAL_UPPER_BOUND: i32 = -INITIAL_LOWER_BOUND;

const MAX_DEPTH: i32 = BOARD_WIDTH as i32 * BOARD_HEIGHT as i32;

struct PVSResult {
    best_move: Option<Move>,
    rating: i32
}

pub struct PVSMoveGetter<Heuristic: Rater> {
    phantom: PhantomData<Heuristic>,
    fixed_depth: bool
}

impl<Heuristic: Rater> PVSMoveGetter<Heuristic> {
    pub fn new() -> Self {
        Self {phantom: PhantomData, fixed_depth: false}
    }

    pub fn new_fixed() -> Self {
        Self {phantom: PhantomData, fixed_depth: true}
    }

    fn pvs(game_state: State, depth: i32, lower_bound: i32, upper_bound: i32, time_measurer: &TimeMeasurer) -> anyhow::Result<PVSResult> {
        if depth < 0 || game_state.is_over() || !time_measurer.has_time_left() {
            return Ok(PVSResult {
                best_move: None,
                rating: Heuristic::rate(&game_state)
            });
        }
        let mut possible_moves = game_state.possible_moves_by_move_generator::<OrderedMoveGenerator<FishDifferenceRater>>();
        let mut best_move = possible_moves.next();
        let mut best_score = lower_bound;
        match best_move.clone() {
            None => {
                best_score = -Self::pvs(game_state.with_moveless_player_skipped()?, depth - 1, -upper_bound, -best_score, time_measurer)?.rating;
            },
            Some(first_move) => {
                let next_game_state = game_state.with_move_performed(first_move.clone())?;
                best_score = -Self::pvs(next_game_state, depth - 1, -upper_bound, -best_score, time_measurer)?.rating;
            }
        }
        for current_move in possible_moves {
            let next_game_state = game_state.with_move_performed(current_move.clone())?;
            let mut current_score: i32 = -Self::pvs(next_game_state.clone(), depth - 1, -best_score - 1, -best_score, time_measurer)?.rating; // zero-window search
            if current_score > lower_bound && current_score < upper_bound {
                // detailed search if zero-window search passes
                current_score = -Self::pvs(next_game_state, depth - 1, -upper_bound, -current_score, time_measurer)?.rating;
            }

            if current_score > best_score {
                best_move = Some(current_move.clone());
                best_score = current_score;
                if current_score >= upper_bound {
                    break;
                }
            }
        }
        Ok(PVSResult {
            best_move,
            rating: best_score
        })
    }
}

impl<Heuristic: Rater> MoveGetter for PVSMoveGetter<Heuristic> {
    fn get_move(&self, state: &State, time_measurer: &TimeMeasurer) -> anyhow::Result<Move> {
        if !state.has_team_any_moves(state.current_team()) {
            anyhow::bail!("MoveGetter invoked without possible moves!");
        }
        if self.fixed_depth {
            return Self::pvs(state.clone(), 1, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, time_measurer).map(|result| result.best_move.unwrap());
        }
        let mut depth = 1; // Skipping 0 because the calculation time of 1 is insignificant
        let mut best_move = Some(state.possible_moves().next().unwrap());
        while time_measurer.has_time_left() {
            let result = Self::pvs(state.clone(), depth, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, time_measurer)?;
            println!("Found move {:?} with rating {} at depth {}", 
                     result.best_move, result.rating, depth);
            best_move = result.best_move;

            if !time_measurer.has_time_left() {
                break;
            }

            if depth >= MAX_DEPTH {
                break;
            }
            depth = depth + 1;
        }
        println!("Reached depth: {}", depth);
        best_move.context("No move found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rostware23_lib::xml::common::Team;
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::board::*;

    use crate::logic::battle::Battle;
    use crate::logic::combined_rater::CombinedRater;
    use crate::logic::fish_difference_rater::FishDifferenceRater;
    use crate::logic::random_getter::*;

    #[test]
    fn given_game_state_with_option_of_either_one_or_two_fish_when_calculating_move_with_zero_depth_then_choose_more_fish() {
        let mut board = Board::empty();
        let moving_penguin_coord = Coordinate::new(12, 0);
        let expected_target = Coordinate::new(10, 0);
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(moving_penguin_coord.clone()), Team::One).unwrap();
        board.set(expected_target.clone(), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(14, 0), FieldState::Fish(1)).unwrap();
        let game_state = State::from_initial_board_with_start_team_one(board);
        let expected_move = Move::Normal{from: moving_penguin_coord, to: expected_target};
        let time_measurer = TimeMeasurer::new_infinite();
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 0, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, &time_measurer).unwrap();
        assert_eq!(expected_move, result_got.best_move.unwrap());
    }

    #[test]
    fn given_game_state_with_option_of_either_one_or_two_or_three_fish_when_calculating_move_with_zero_depth_then_choose_more_fish() {
        let mut board = Board::empty();
        let moving_penguin_coord = Coordinate::new(12, 0);
        let expected_target = Coordinate::new(11, 1);
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(moving_penguin_coord.clone()), Team::One).unwrap();
        board.set(expected_target.clone(), FieldState::Fish(3)).unwrap();
        board.set(Coordinate::new(10, 0), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(14, 2), FieldState::Fish(1)).unwrap();
        let game_state = State::from_initial_board_with_start_team_one(board);
        let expected_move = Move::Normal{from: moving_penguin_coord, to: expected_target};
        let time_measurer = TimeMeasurer::new_infinite();
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 0, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, &time_measurer).unwrap();
        assert_eq!(expected_move, result_got.best_move.unwrap());
    }

    #[test]
    fn pvs_move_getter_wins_most_games_vs_random_move_getter() {
        let random_getter = RandomGetter::new();
        let pvs_getter = PVSMoveGetter::<CombinedRater>::new_fixed();
        let playout = Battle::between(&random_getter, &pvs_getter);
        let result_1 = playout.multiple_bi_directional(3).unwrap();
        assert_eq!(result_1.winner(), Some(Team::Two));
    }

    fn create_higher_depth_test_game_state(moving_penguin_coord: Coordinate, expected_target: Coordinate) -> State {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(moving_penguin_coord.clone()), Team::One).unwrap();
        board.set(expected_target.clone(), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(14, 0), FieldState::Fish(2)).unwrap();
        board.set(Coordinate::new(15, 1), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(9, 1), FieldState::Fish(4)).unwrap();
        board.set(Coordinate::new(4, 4), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(5, 5), FieldState::Fish(1)).unwrap();
        State::from_initial_board_with_start_team_one(board)
    }

    #[test]
    fn given_game_state_with_option_of_either_one_then_four_or_two_then_one_fish_and_also_one_fish_for_opponent_when_selecting_best_move_with_depth_two_then_choose_one_to_gain_fish() {
        let moving_penguin_coord = Coordinate::new(12, 0);
        let expected_target = Coordinate::new(10, 0);
        let game_state = create_higher_depth_test_game_state(moving_penguin_coord.clone(), expected_target.clone());
        let expected_move = Move::Normal{from: moving_penguin_coord, to: expected_target};
        let time_measurer = TimeMeasurer::new_infinite();
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 2, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, &time_measurer).unwrap();
        assert_eq!(expected_move, result_got.best_move.unwrap());
    }

    #[test]
    fn given_game_state_with_option_of_either_one_then_four_or_two_then_one_fish_and_also_one_fish_for_opponent_with_aspiration_window_when_selecting_best_move_with_depth_two_then_choose_one_to_gain_fish() {
        let moving_penguin_coord = Coordinate::new(12, 0);
        let expected_target = Coordinate::new(10, 0);
        let game_state = create_higher_depth_test_game_state(moving_penguin_coord.clone(), expected_target.clone());
        let expected_move = Move::Normal{from: moving_penguin_coord, to: expected_target};
        let time_measurer = TimeMeasurer::new_infinite();
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 2, 3, 5, &time_measurer).unwrap();
        assert_eq!(expected_move, result_got.best_move.unwrap());
    }

    #[test]
    fn given_game_state_with_option_of_either_one_then_four_or_two_then_one_fish_and_also_one_fish_for_opponent_with_wrong_aspiration_window_when_selecting_best_move_with_depth_two_then_return_upper_bound_or_higher_value_as_rating() {
        let moving_penguin_coord = Coordinate::new(12, 0);
        let expected_target = Coordinate::new(10, 0);
        let game_state = create_higher_depth_test_game_state(moving_penguin_coord.clone(), expected_target.clone());
        let time_measurer = TimeMeasurer::new_infinite();
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 2, 0, 2, &time_measurer).unwrap();
        assert!(2 <= result_got.rating);
    }

    #[test]
    fn can_calculate_correct_rating_with_higher_depth_when_opponent_doesnt_have_any_moves() {
        let mut board = Board::empty();
        for i in 0..5 {
            board.perform_move(Move::Place(Coordinate::new(i*2, 0)), Team::One).unwrap();
            board.perform_move(Move::Place(Coordinate::new(i*2, 4)), Team::Two).unwrap();
        }
        board.set(Coordinate::new(10, 0), FieldState::Fish(2)).unwrap();
        let game_state = State::from_initial_board_with_start_team_one(board);
        let time_measurer = TimeMeasurer::new_infinite();
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 2, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, &time_measurer).unwrap();
        assert_eq!(2, result_got.rating);
    }
}
