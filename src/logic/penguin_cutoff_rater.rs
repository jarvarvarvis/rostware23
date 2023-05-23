use rostware23_lib::game::direction::DirectionIterator;
use rostware23_lib::game::state::State;
use rostware23_lib::xml::common::Team;

use super::Rater;

pub struct PenguinCutOffRater;

impl PenguinCutOffRater {
    fn get_rating_for_possible_targets_count(targets_count: i32) -> i32 {
        match targets_count {
            0 => 50,
            1 => 2,
            2 => 1,
            _ => 0
        }
    }

    fn get_team_based_prefix(state: &State, penguin_team: Team) -> i32 {
        if state.current_team() == penguin_team {
            -1
        } else {
            1
        }
    }

    fn get_cutoff_rating_for_team(state: &State, team: Team) -> i32 {
        let prefix = Self::get_team_based_prefix(state, team.clone());
        let mut rating = 0;
        let penguins = state.board.get_penguin_iterator(team);
        for penguin in penguins {
            let penguin_coords = penguin.coordinate;
            let possible_targets = DirectionIterator::new()
                .map(|direction| penguin_coords.add(direction.vector()))
                .filter(|coordinate|
                    state.board.can_move_to(coordinate.clone()).unwrap_or(false));
            let possible_targets_count = possible_targets.count() as i32;
            rating += prefix * Self::get_rating_for_possible_targets_count(possible_targets_count);
        }

        rating
    }
}

impl Rater for PenguinCutOffRater {
    fn rate(state: &State) -> i32 {
        let current_team = state.current_team();
        Self::get_cutoff_rating_for_team(state, current_team.clone()) +
            Self::get_cutoff_rating_for_team(state, current_team.opponent())
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::game::board::Board;
    use rostware23_lib::xml::state::FieldState;

    use super::*;

    #[test]
    fn cutoff_rating_for_team_one_when_it_has_been_cut_off() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::Two).unwrap();
        board.set(Coordinate::new(4, 0), FieldState::Fish(1)).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let actual = PenguinCutOffRater::get_cutoff_rating_for_team(&state, Team::One);
        assert_eq!(-50, actual);
    }

    #[test]
    fn cutoff_rating_for_team_two_when_team_one_has_been_cut_off() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::Two).unwrap();
        board.set(Coordinate::new(4, 0), FieldState::Fish(1)).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let actual = PenguinCutOffRater::get_cutoff_rating_for_team(&state, Team::Two);
        assert_eq!(2, actual);
    }

    #[test]
    fn cutoff_rating_for_team_one_when_it_is_about_to_be_cut_off() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(1, 1)), Team::Two).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let actual = PenguinCutOffRater::get_cutoff_rating_for_team(&state, Team::One);
        assert_eq!(-2, actual);
    }

    #[test]
    fn cutoff_rating_for_team_one_with_multiple_penguins_cut_off() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);
        let actual = PenguinCutOffRater::get_cutoff_rating_for_team(&state, Team::One);
        assert_eq!(-150, actual);
    }

    #[test]
    fn cutoff_rating_of_team_one_when_team_two_penguins_have_been_or_are_about_to_be_cut_off() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::Two).unwrap();
        board.set(Coordinate::new(6, 0), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(8, 0), FieldState::Fish(1)).unwrap();

        board.perform_move(Move::Place(Coordinate::new(5, 1)), Team::One).unwrap();
        board.set(Coordinate::new(7, 1), FieldState::Fish(1)).unwrap();

        let state = State::from_initial_board_with_start_team_one(board);
        let actual = PenguinCutOffRater::rate(&state);
        let expected = 50 + 2 - 1;
        assert_eq!(expected, actual);
    }
}
