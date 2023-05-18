pub mod random_getter;

use crate::game::state::State;
use crate::game::moves::Move;

pub trait MoveGetter {
    fn get_move(&self, state: &State) -> anyhow::Result<Move>;
}
