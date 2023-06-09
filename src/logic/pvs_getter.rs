use std::marker::PhantomData;

use anyhow::Context;
use rostware23_lib::game::common::{BOARD_WIDTH, BOARD_HEIGHT};
use rostware23_lib::game::moves::Move;
use rostware23_lib::game::state::State;
use crate::logic::selective_transposition_table::SelectiveTranspositionTable;
use crate::logic::simple_transposition_table::SimpleTranspositionTable;
use crate::logic::state_selector::{AnyStateSelector, StateSelector};
use crate::logic::transposition_table::TranspositionTable;

use super::MoveGetter;
use super::fish_difference_rater::FishDifferenceRater;
use super::ordered_move_generator::OrderedMoveGenerator;
use super::time_measurer::TimeMeasurer;
use super::Rater;

const INITIAL_LOWER_BOUND: i32 = -1000000;
const INITIAL_UPPER_BOUND: i32 = -INITIAL_LOWER_BOUND;

const INITIAL_OFFSET: i32 = 12;
const WIDENING_FACTOR: i32 = 3;

const MAX_DEPTH: i32 = BOARD_WIDTH as i32 * BOARD_HEIGHT as i32;

fn is_in_search_window(value: i32, lower_bound: i32, upper_bound: i32) -> bool {
    lower_bound < value && value < upper_bound
}

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

    fn pvs(game_state: State, depth: i32, mut lower_bound: i32, upper_bound: i32, time_measurer: &TimeMeasurer, transposition_table: &mut SelectiveTranspositionTable<SimpleTranspositionTable, AnyStateSelector>) -> anyhow::Result<PVSResult> {
        if transposition_table.contains(&game_state) {
            return Ok(PVSResult {
                best_move: None,
                rating: transposition_table.get(&game_state)?
            })
        }
        if depth < 0 || game_state.is_over() || !time_measurer.has_time_left() {
            let rating = Heuristic::rate(&game_state);
            transposition_table.add(game_state, rating);
            return Ok(PVSResult {
                best_move: None,
                rating
            });
        }
        let mut possible_moves = game_state.possible_moves_by_move_generator::<OrderedMoveGenerator<FishDifferenceRater>>();
        let mut best_move = possible_moves.next();
        let mut best_score;
        match best_move.clone() {
            None => {
                best_score = -Self::pvs(game_state.with_moveless_player_skipped()?, depth, -upper_bound, -lower_bound, time_measurer, transposition_table)?.rating;
                if best_score > lower_bound && best_score < upper_bound {
                    lower_bound = best_score;
                }
            },
            Some(first_move) => {
                let next_game_state = game_state.with_move_performed(first_move.clone())?;
                best_score = -Self::pvs(next_game_state, depth - 1, -upper_bound, -lower_bound, time_measurer, transposition_table)?.rating;
                if best_score > lower_bound && best_score < upper_bound {
                    lower_bound = best_score;
                }
            }
        }

        for current_move in possible_moves {
            let next_game_state = game_state.with_move_performed(current_move.clone())?;

            // Zero-window search
            let mut current_score: i32 = -Self::pvs(next_game_state.clone(), depth - 1, -lower_bound - 1, -lower_bound, time_measurer, transposition_table)?.rating;
            if current_score > lower_bound && current_score < upper_bound {
                // Detailed search if zero-window search passes
                current_score = -Self::pvs(next_game_state, depth - 1, -upper_bound, -lower_bound, time_measurer, transposition_table)?.rating;
                if current_score > lower_bound {
                    lower_bound = current_score;
                }
            }

            if current_score > best_score {
                best_move = Some(current_move.clone());
                best_score = current_score;
                if current_score >= upper_bound {
                    break;
                }
            }
        }
        if best_score >= lower_bound && best_score < upper_bound {
            transposition_table.add(game_state, best_score);
        }
        Ok(PVSResult {
            best_move,
            rating: best_score
        })
    }

    fn get_move_for_depth(&self, state: &State, depth: i32, last_rating: i32, time_measurer: &TimeMeasurer) -> anyhow::Result<PVSResult> {
        let mut offset_lower_bound = -INITIAL_OFFSET;
        let mut offset_upper_bound = INITIAL_OFFSET;
        let mut lower_bound = last_rating + offset_lower_bound;
        let mut upper_bound = last_rating + offset_upper_bound;
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(depth);
        while time_measurer.has_time_left() {
            let current_result = Self::pvs(state.clone(), depth, lower_bound, upper_bound, time_measurer, &mut transposition_table)?;
            let current_rating = current_result.rating;
            if is_in_search_window(current_rating, lower_bound, upper_bound) {
                println!("Search Window (Depth = {}): [{} {}]", depth, lower_bound, upper_bound);
                return Ok(current_result);
            }

            if current_rating <= lower_bound {
                upper_bound = lower_bound;
                offset_lower_bound *= WIDENING_FACTOR;
                lower_bound = last_rating + offset_lower_bound;
            } else {
                lower_bound = upper_bound;
                offset_upper_bound *= WIDENING_FACTOR;
                upper_bound = last_rating + offset_upper_bound;
            }
        }

        Ok(PVSResult {
            best_move: None,
            rating: i32::min_value()
        })
    }
}

impl<Heuristic: Rater> MoveGetter for PVSMoveGetter<Heuristic> {
    fn get_move(&self, state: &State, time_measurer: &TimeMeasurer) -> anyhow::Result<Move> {
        if !state.has_team_any_moves(state.current_team()) {
            anyhow::bail!("MoveGetter invoked without possible moves!");
        }
        
        if self.fixed_depth {
            let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(1);
            return Self::pvs(state.clone(), 1, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, time_measurer, &mut transposition_table).map(|result| result.best_move.unwrap());
        }

        let mut depth = 1; // Skipping 0 because the calculation time of 1 is insignificant
        let mut best_move = Some(state.possible_moves().next().unwrap());
        let mut last_ratings = [Heuristic::rate(state); 2];
        while time_measurer.has_time_left() {
            let depth_index = (depth % 2) as usize;
            let current_result = self.get_move_for_depth(state, depth, last_ratings[depth_index], time_measurer)?;
            last_ratings[depth_index] = current_result.rating;

            if current_result.best_move.is_none() {
                continue;
            }

            if !time_measurer.has_time_left() {
                break;
            }

            if depth >= MAX_DEPTH {
                break;
            }

            best_move = current_result.best_move;
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
    use crate::logic::bitset_penguin_restrictions::BitsetPenguinRestrictions;
    use crate::logic::board_parser::parse_board;
    use crate::logic::combined_rater::CombinedRater;
    use crate::logic::fish_difference_rater::FishDifferenceRater;
    use crate::logic::penguin_cutoff_rater::PenguinCutOffRater;
    use crate::logic::potential_fish_rater::PotentialFishRater;
    use crate::logic::quadrant_occupation_rater::QuadrantOccupationRater;
    use crate::logic::random_getter::*;
    use crate::logic::restricted_reachable_fish_rater::RestrictedReachableFishRater;

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
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(0);
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 0, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, &time_measurer, &mut transposition_table).unwrap();
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
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(0);
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 0, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, &time_measurer, &mut transposition_table).unwrap();
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
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(2);
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, 2, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, &time_measurer, &mut transposition_table).unwrap();
        assert_eq!(expected_move, result_got.best_move.unwrap());
    }

    #[test]
    fn given_game_state_with_option_of_either_one_then_four_or_two_then_one_fish_and_also_one_fish_for_opponent_with_aspiration_window_when_selecting_best_move_with_depth_two_then_choose_one_to_gain_fish() {
        let moving_penguin_coord = Coordinate::new(12, 0);
        let expected_target = Coordinate::new(10, 0);
        let game_state = create_higher_depth_test_game_state(moving_penguin_coord.clone(), expected_target.clone());
        let expected_move = Move::Normal{from: moving_penguin_coord, to: expected_target};
        let time_measurer = TimeMeasurer::new_infinite();
        let depth = 2;
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(depth);
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, depth, 3, 5, &time_measurer, &mut transposition_table).unwrap();
        assert_eq!(expected_move, result_got.best_move.unwrap());
    }

    #[test]
    fn given_game_state_with_option_of_either_one_then_four_or_two_then_one_fish_and_also_one_fish_for_opponent_with_wrong_aspiration_window_when_selecting_best_move_with_depth_two_then_return_upper_bound_or_higher_value_as_rating() {
        let moving_penguin_coord = Coordinate::new(12, 0);
        let expected_target = Coordinate::new(10, 0);
        let game_state = create_higher_depth_test_game_state(moving_penguin_coord.clone(), expected_target.clone());
        let time_measurer = TimeMeasurer::new_infinite();
        let depth = 2;
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(depth);
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, depth, 0, 2, &time_measurer, &mut transposition_table).unwrap();
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
        let depth = 2;
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(depth);
        let result_got: PVSResult = PVSMoveGetter::<FishDifferenceRater>::pvs(game_state, depth, INITIAL_LOWER_BOUND, INITIAL_UPPER_BOUND, &time_measurer, &mut transposition_table).unwrap();
        assert_eq!(2, result_got.rating);
    }

    #[test]
    fn early_game_cut_off_test() {
        let board_string = "\
            = - 4 = 3 -   3\n\
             = - P 3 G = = -\n\
             \x20  =   G = = = =\n\
             -     = - P -  \n\
             \x20  - -   =     -\n\
             = = = =     =  \n\
            - = P G G P - =\n\
             3   - 3 = 4 - =\n";
        let board = parse_board(board_string);
        println!("{}", &board);
        let game_state = State::from_initial_board_with_start_team_one(board);
        let pvs_move_getter = PVSMoveGetter::<CombinedRater>::new();
        let expected_move = Move::Normal{from: Coordinate::new(6, 6), to: Coordinate::new(4, 4) };
        let move_got = pvs_move_getter.get_move_for_depth(&game_state, 3, 0, &TimeMeasurer::Infinite).unwrap().best_move.unwrap();
        assert_eq!(expected_move, move_got)
    }
}
