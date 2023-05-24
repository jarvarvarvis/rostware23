use rostware23_lib::game::common::{Coordinate, RIGHTMOST_X, BOARD_HEIGHT};
use rostware23_lib::game::state::State;

use super::Rater;

pub struct EdgePenguinPenalty;

impl EdgePenguinPenalty {
    fn rate_penguin_coordinate(coordinate: Coordinate) -> i32 {
        let x = coordinate.x() / 2;
        let y = coordinate.y();
        let on_edge = 
            x == 0 || x == RIGHTMOST_X / 2 ||
            y == 0 || y == BOARD_HEIGHT - 1;
        on_edge as i32
    }
}

impl Rater for EdgePenguinPenalty {
    fn rate(state: &State) -> i32 {
        let current_team = state.current_team();
        let own_penguins = state.board.get_penguin_iterator(current_team.clone());
        let own_rating = own_penguins
            .map(|penguin| Self::rate_penguin_coordinate(penguin.coordinate))
            .sum::<i32>();
        let opponent_penguins = state.board.get_penguin_iterator(current_team.opponent());
        let opponent_rating = opponent_penguins
            .map(|penguin| Self::rate_penguin_coordinate(penguin.coordinate))
            .sum::<i32>();
        opponent_rating - own_rating
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::board::Board;
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::common::Team;
    use rostware23_lib::xml::state::FieldState;

    use super::*;

    #[test]
    fn rate_penguins_of_both_teams_on_simple_board() {
        let mut board = Board::fill(FieldState::Fish(1));
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(1, 1)), Team::One).unwrap();

        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(15, 1)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(1, 7)), Team::Two).unwrap();

        let state = State::from_initial_board_with_start_team_one(board);
        assert_eq!(1, EdgePenguinPenalty::rate(&state));
    }
}
