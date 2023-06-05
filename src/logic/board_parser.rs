use rostware23_lib::game::board::{Board, FieldState};
use rostware23_lib::game::common::{Coordinate, Team};
use rostware23_lib::game::moves::Move;
use rostware23_lib::game::state::State;
use crate::logic::bitset_penguin_restrictions::BitsetPenguinRestrictions;
use crate::logic::edge_penguin_penalty::EdgePenguinPenalty;
use crate::logic::fish_difference_rater::FishDifferenceRater;
use crate::logic::penguin_cutoff_rater::PenguinCutOffRater;
use crate::logic::potential_fish_rater::PotentialFishRater;
use crate::logic::quadrant_occupation_rater::QuadrantOccupationRater;
use crate::logic::Rater;
use crate::logic::restricted_reachable_fish_rater::RestrictedReachableFishRater;
use crate::logic::staged_rater::StagedRater;
use crate::logic::vec_penguin_restrictions::VecPenguinRestrictions;

pub fn parse_board(board_string: &str) -> Board {
    let mut board = Board::empty();
    let mut x = 0;
    let mut y = 0;
    for c in board_string.chars() {
        let coord = Coordinate::new(x, y);
        match c {
            'G' => board.perform_move(Move::Place(coord), Team::One).unwrap(),
            'P' => board.perform_move(Move::Place(coord), Team::Two).unwrap(),
            '-' => board.set(coord, FieldState::Fish(1)).unwrap(),
            '=' => board.set(coord, FieldState::Fish(2)).unwrap(),
            '3' => board.set(coord, FieldState::Fish(3)).unwrap(),
            '4' => board.set(coord, FieldState::Fish(4)).unwrap(),
            _ => {}
        }
        x += 1;
        if c == '\n' {
            y += 1;
            x = 0;
        }
    }
    board
}

#[test]
fn board_parser_manual_test() {
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
    println!("{}", board);
    let game_state = State::from_initial_board_with_start_team_one(board);
    println!("Rating: {}",
        RestrictedReachableFishRater::<BitsetPenguinRestrictions>::rate(&game_state)
    )
}