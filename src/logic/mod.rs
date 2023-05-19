pub mod random_getter;
pub mod pvs_getter;

use rostware23_lib::game::state::State;
use rostware23_lib::game::moves::Move;

pub trait MoveGetter {
    fn get_move(&self, state: &State) -> anyhow::Result<Move>;
}
