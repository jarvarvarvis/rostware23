use super::board::FieldState;
use super::common::*;
use super::move_generator::MoveGenerator;
use super::moves::*;
use super::penguin::PenguinPossibleMoveIterator;
use super::state::*;

const MAX_PENGUIN_COUNT_FOR_SINGLE_TEAM: usize = 4;

pub struct PossibleMovesIterator {
    move_iter: Box<dyn Iterator<Item = Move>>
}

impl PossibleMovesIterator {
    pub fn empty() -> Self {
        Self {
            move_iter: Box::new(vec![].into_iter())
        }
    }

    pub fn from_state_and_team(state: State, team: Team) -> Self {
        let penguins_placed = state.board.get_penguin_iterator(team).count();
        if penguins_placed >= MAX_PENGUIN_COUNT_FOR_SINGLE_TEAM {
            Self::make_normal_moves_iterator_for_team(state, team)
        } else {
            Self::make_beginning_place_moves_iterator(state)
        }
    }

    fn make_normal_moves_iterator_for_team(state: State, team: Team) -> Self {
        let penguin_iterator = state.board.get_penguin_iterator(team)
            .flat_map(move |penguin| PenguinPossibleMoveIterator::from(penguin, state.board.clone()));

        Self {
            move_iter: Box::new(penguin_iterator)
        }
    }

    fn make_beginning_place_moves_iterator(state: State) -> Self {
        let board_coordinate_iterator = BoardCoordinateIterator::new();
        let moves = board_coordinate_iterator
            .filter(move |coordinate| match state.board.get(coordinate.clone()) {
                Ok(FieldState::Fish(1)) => true,
                _ => false,
            })
            .map(|target| Move::Place(target));
        Self {
            move_iter: Box::new(moves)
        }
    }
}

impl From<State> for PossibleMovesIterator {
    fn from(state: State) -> Self {
        match state.current_team() {
            Ok(team) => Self::from_state_and_team(state, team),
            Err(_) => Self::empty()
        }
    }
}

impl Iterator for PossibleMovesIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.move_iter.next()
    }
}

impl MoveGenerator for PossibleMovesIterator {
    type MoveIterator = PossibleMovesIterator;

    fn get_possible_moves(state: State) -> Self::MoveIterator {
        PossibleMovesIterator::from(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::board::Board;

    use super::*;

    #[test]
    fn possible_moves_iterator_gives_64_possible_moves_on_all_1_fish_board() {
        let state = State::from_initial_board_with_start_team_one(Board::fill(FieldState::Fish(1)));
        let possible_moves_iter = PossibleMovesIterator::from(state);
        assert_eq!(64, possible_moves_iter.count());
    }

    #[test]
    fn possible_moves_iterator_creation_from_state_fails_on_empty_board() {
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        let possible_moves_iter = PossibleMovesIterator::from(state);
        assert_eq!(0, possible_moves_iter.count());
    }
    
    #[test]
    fn possible_moves_iterator_gives_correct_moves_on_simple_board() {
        let mut board = Board::empty();
        board.set(Coordinate::new(0, 0), FieldState::Fish(2)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 0)), Team::One).unwrap();
        board.set(Coordinate::new(8, 0), FieldState::Fish(2)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(12, 0)), Team::One).unwrap();
        
        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(8, 2)), Team::Two).unwrap();
        let state = State::from_initial_board_with_start_team_one(board);

        println!("{}", state);

        let mut team_one_possible_moves_iter = PossibleMovesIterator::from_state_and_team(state.clone(), Team::One);
        assert_eq!(Move::Normal { from: Coordinate::new(2, 0), to: Coordinate::new(0, 0) }, team_one_possible_moves_iter.next().unwrap());
        assert_eq!(Move::Normal { from: Coordinate::new(6, 0), to: Coordinate::new(8, 0) }, team_one_possible_moves_iter.next().unwrap());
        assert_eq!(None, team_one_possible_moves_iter.next());

        let mut team_two_possible_moves_iter = PossibleMovesIterator::from_state_and_team(state, Team::Two);
        assert_eq!(None, team_two_possible_moves_iter.next());
    }
}
