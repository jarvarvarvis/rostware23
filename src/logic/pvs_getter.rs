use super::MoveGetter;
use rostware23_lib::game::moves::Move;
use rostware23_lib::game::common::Coordinate;
use rostware23_lib::game::board::Board;
use rostware23_lib::game::state::State;
use rostware23_lib::game::board::FieldState;
use rostware23_lib::xml::common::Team;

struct PVSResult {
    best_move: Move,
    rating: i32
}

pub struct PVSMoveGetter {
}

impl PVSMoveGetter {
    fn pvs(game_state: State) -> PVSResult {
        let mut best_move = Move::Place(Coordinate::new(17, 9));
        let mut best_score = i32::min_value();
        let mut possible_moves = game_state.possible_moves();
        for current_move in possible_moves {
            let current_score: i32 = game_state.board.get(current_move.get_to()).unwrap().get_fish_count().unwrap() as i32;
            if current_score > best_score {
                best_move = current_move.clone();
                best_score = current_score;
            }
        }
        PVSResult {
            best_move: best_move,
            rating: best_score
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let result_got: PVSResult = PVSMoveGetter::pvs(game_state);
        assert_eq!(expected_move, result_got.best_move);
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
        let result_got: PVSResult = PVSMoveGetter::pvs(game_state);
        assert_eq!(expected_move, result_got.best_move);
    }
}
