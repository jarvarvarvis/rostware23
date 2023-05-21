pub mod battle;
pub mod random_getter;
pub mod pvs_getter;
pub mod fish_difference_rater;
pub mod potential_fish_rater;
pub mod reachable_fish_rater;
pub mod combined_rater;

use rostware23_lib::game::state::State;
use rostware23_lib::game::moves::Move;

pub trait MoveGetter {
    fn get_move(&self, state: &State) -> anyhow::Result<Move>;
}

pub trait Rater {
    fn rate(state: &State) -> i32;
}
