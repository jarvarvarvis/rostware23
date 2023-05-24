use std::marker::PhantomData;

use rostware23_lib::game::{move_generator::MoveGenerator, moves::Move};

use super::Rater;

pub struct OrderedMoveGenerator<Heuristic: Rater> {
    phantom: PhantomData<Heuristic>
}

impl<Heuristic: Rater> MoveGenerator for OrderedMoveGenerator<Heuristic> {
    type MoveIterator = std::vec::IntoIter<Move>;

    fn get_possible_moves(state: rostware23_lib::game::state::State) -> Self::MoveIterator {
        let mut possible_moves: Vec<Move> = state.possible_moves().collect();
        possible_moves.sort_unstable_by(|a, b| {
            let rating_a = Heuristic::rate(&state.with_move_performed(a.clone()).unwrap());
            let rating_b = Heuristic::rate(&state.with_move_performed(b.clone()).unwrap());
            rating_a.cmp(&rating_b)
        });
        possible_moves.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::board::Board;
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::state::State;
    use rostware23_lib::xml::common::Team;
    use rostware23_lib::xml::state::FieldState;

    use crate::logic::fish_difference_rater::FishDifferenceRater;

    use super::*;

    #[test]
    fn on_simple_game_state_ordered_move_generator_with_fish_difference_rater_sorts_move_to_field_with_more_fish_first() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        board.set(Coordinate::new(2, 0), FieldState::Fish(1)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.set(Coordinate::new(6, 0), FieldState::Fish(4)).unwrap();

        board.perform_move(Move::Place(Coordinate::new(0, 2)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::One).unwrap();

        let state = State::from_initial_board_with_start_team_one(board);
        println!("{state}");
        let mut possible_moves = state.possible_moves_by_move_generator::<OrderedMoveGenerator<FishDifferenceRater>>();
        assert_eq!(Move::Normal { from: Coordinate::new(4, 0), to: Coordinate::new(6, 0) }, possible_moves.next().unwrap()); // 4 Fish
        assert_eq!(Move::Normal { from: Coordinate::new(0, 0), to: Coordinate::new(2, 0) }, possible_moves.next().unwrap()); // 1 Fish
        assert_eq!(Move::Normal { from: Coordinate::new(4, 0), to: Coordinate::new(2, 0) }, possible_moves.next().unwrap()); // 1 Fish
        assert_eq!(None, possible_moves.next());
    }
}
