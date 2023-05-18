use super::board::{Board, FieldState};
use super::common::*;
use super::moves::*;
use super::state::*;

const MAX_PENGUIN_COUNT_FOR_SINGLE_TEAM: usize = 4;

pub struct PossibleMovesIterator {
    move_iter: Box<dyn Iterator<Item = Move>>
}

impl PossibleMovesIterator {
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
        let team = state.current_team();
        let penguins_placed = state.board.get_penguin_iterator(team).count();
        if penguins_placed >= MAX_PENGUIN_COUNT_FOR_SINGLE_TEAM {
            unimplemented!("Normal move generation is not implemented") 
        } else {
            Self::make_beginning_place_moves_iterator(state)
        }
    }
}

impl Iterator for PossibleMovesIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.move_iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_moves_iterator_gives_64_possible_moves_on_all_1_fish_board() {
        let state = State::from_initial_board(Board::fill(FieldState::Fish(1)));
        let possible_moves_iter = PossibleMovesIterator::from(state);
        assert_eq!(64, possible_moves_iter.count());
    }

    #[test]
    fn possible_moves_iterator_gives_no_possible_moves_on_empty_board() {
        let state = State::from_initial_board(Board::empty());
        let possible_moves_iter = PossibleMovesIterator::from(state);
        assert_eq!(0, possible_moves_iter.count());
    }
}
