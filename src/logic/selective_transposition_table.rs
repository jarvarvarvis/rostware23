use std::marker::PhantomData;

use rostware23_lib::game::state::State;

use super::state_selector::StateSelector;
use super::transposition_table::TranspositionTable;

pub struct SelectiveTranspositionTable<Table: TranspositionTable, Selector: StateSelector> {
    inner: Table,
    phantom: PhantomData<Selector>
}

impl<Table: TranspositionTable, Selector: StateSelector> TranspositionTable
    for SelectiveTranspositionTable<Table, Selector>
{
    fn create_for_depth(depth: i32) -> Self {
        Self {
            inner: Table::create_for_depth(depth),
            phantom: PhantomData
        }
    }

    fn add(&mut self, state: State, rating: i32) {
        if Selector::should_be_saved(&state) {
            self.inner.add(state, rating);
        }
    }

    fn contains(&self, state: &State) -> bool {
        self.inner.contains(state)
    }

    fn get(&self, state: &State) -> anyhow::Result<i32> {
        self.inner.get(state)
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use rostware23_lib::game::board::Board;
    use rostware23_lib::game::common::Coordinate;
    use rostware23_lib::game::moves::Move;
    use rostware23_lib::xml::common::Team;
    use rostware23_lib::xml::state::FieldState;

    use crate::logic::simple_transposition_table::SimpleTranspositionTable;
    use crate::logic::state_selector::{AnyStateSelector, NoStateSelector};

    use super::*;

    #[test]
    fn selective_transposition_table_wrapping_simple_transposition_table_for_depth_0_then_inner_has_no_entries() {
        let transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(0);
        assert!(transposition_table.inner.is_empty());
    }

    #[test]
    fn selective_transposition_table_wrapping_simple_transposition_table_for_greater_depth_then_inner_has_no_entries() {
        let transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(9);
        assert!(transposition_table.inner.is_empty());
    }

    #[test]
    fn adding_new_state_with_rating_to_selective_transposition_table_with_any_selector_adds_state() {
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(5);
        let rating = 61;
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        transposition_table.add(state.clone(), rating);

        assert!(transposition_table.inner.get(&state).is_ok());
    }

    #[test]
    fn adding_new_state_with_rating_to_selective_transposition_table_with_no_selector_adds_state() {
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, NoStateSelector>::create_for_depth(4);
        let rating = 43;
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        transposition_table.add(state.clone(), rating);

        assert!(transposition_table.inner.get(&state).is_err());
    }

    #[test]
    fn adding_new_state_with_rating_to_selective_transposition_table_with_any_selector_returns_true_when_checking_if_table_contains_state() {
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(1);
        let rating = 19;
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        transposition_table.add(state.clone(), rating);

        assert!(transposition_table.contains(&state));
    }

    #[test]
    fn adding_new_state_with_rating_to_selective_transposition_table_with_no_selector_returns_true_when_checking_if_table_contains_state() {
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, NoStateSelector>::create_for_depth(4);
        let rating = 25;
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        transposition_table.add(state.clone(), rating);

        assert!(!transposition_table.contains(&state));
    }

    #[test]
    fn adding_some_advanced_state_with_rating_to_selective_transposition_table_with_any_selector_and_getting_added_state_works_correctly() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 6)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(12, 6)), Team::One).unwrap();
        board.set(Coordinate::new(9, 7), FieldState::Fish(1)).unwrap();
        board.set(Coordinate::new(1, 7), FieldState::Fish(1)).unwrap();
        let mut state = State::from_initial_board_with_start_team_one(board);
        state.turn = 3;

        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(1);
        transposition_table.add(state.clone(), 22);

        assert_eq!(22, transposition_table.get(&state).unwrap());
    }

    #[test]
    fn new_selective_transposition_table_with_any_selector_is_empty() {
        let transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(0);
        assert!(transposition_table.is_empty());
    }

    #[test]
    fn new_selective_transposition_table_with_no_selector_is_empty() {
        let transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, NoStateSelector>::create_for_depth(3);
        assert!(transposition_table.is_empty());
    }

    #[test]
    fn selective_transposition_table_with_any_selector_with_state_added_is_not_empty() {
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, AnyStateSelector>::create_for_depth(2);
        transposition_table.add(State::from_initial_board_with_start_team_one(Board::empty()), 1);
        assert!(!transposition_table.is_empty());
    }

    #[test]
    fn selective_transposition_table_with_no_selector_with_state_added_is_empty() {
        let mut transposition_table = SelectiveTranspositionTable::<SimpleTranspositionTable, NoStateSelector>::create_for_depth(4);
        transposition_table.add(State::from_initial_board_with_start_team_one(Board::empty()), 1);
        assert!(transposition_table.is_empty());
    }
}
