use super::moves::Move;
use super::state::State;

pub trait MoveGenerator {
    type MoveIterator: Iterator<Item = Move>;
    fn get_possible_moves(state: State) -> Self::MoveIterator;
}
