use std::collections::HashMap;

use anyhow::Context;
use rostware23_lib::game::state::State;

use super::transposition_table::TranspositionTable;

pub struct SimpleTranspositionTable {
    entries: HashMap<State, i32>
}

impl TranspositionTable for SimpleTranspositionTable {
    fn create_for_depth(_: i32) -> Self {
        Self { entries: HashMap::new() }
    }

    fn add(&mut self, state: State, rating: i32) {
        self.entries.insert(state, rating);
    }

    fn contains(&self, state: &State) -> bool {
        self.entries.contains_key(state)
    }

    fn get(&self, state: &State) -> anyhow::Result<i32> {
        self.entries.get(state)
            .context(format!("State not present in the transposition table:\n{}", state))
            .map(|value| *value)
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::{game::{board::Board, common::Coordinate, moves::Move}, xml::{common::Team, state::FieldState}};

    use super::*;

    #[test]
    fn simple_transposition_table_for_depth_0_has_no_entries() {
        let transposition_table = SimpleTranspositionTable::create_for_depth(0);
        assert!(transposition_table.entries.is_empty());
    }

    #[test]
    fn simple_transposition_table_for_greater_depth_has_no_entries() {
        let transposition_table = SimpleTranspositionTable::create_for_depth(5);
        assert!(transposition_table.entries.is_empty());
    }

    #[test]
    fn adding_new_state_with_rating_to_simple_transposition_table_works_correctly() {
        let mut transposition_table = SimpleTranspositionTable::create_for_depth(2);
        let rating = 136;
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        transposition_table.add(state.clone(), rating);

        let expected = HashMap::from([
            (state, rating)
        ]);

        assert_eq!(expected, transposition_table.entries);
    }

    #[test]
    fn adding_new_state_with_rating_to_simple_transposition_table_and_checking_if_the_transposition_table_contains_added_value_returns_true() {
        let mut transposition_table = SimpleTranspositionTable::create_for_depth(1);
        let rating = 40;
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        transposition_table.add(state.clone(), rating);

        assert!(transposition_table.contains(&state));
    }

    #[test]
    fn adding_some_advanced_state_with_rating_to_simple_transposition_table_and_getting_added_state_works_correctly() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(1, 3)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(11, 5)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(8, 4)), Team::Two).unwrap();
        board.set(Coordinate::new(7, 7), FieldState::Fish(3)).unwrap();
        let mut state = State::from_initial_board_with_start_team_one(board);
        state.turn = 3;

        let mut transposition_table = SimpleTranspositionTable::create_for_depth(3);
        transposition_table.add(state.clone(), 22);

        assert_eq!(22, transposition_table.get(&state).unwrap());
    }
}

