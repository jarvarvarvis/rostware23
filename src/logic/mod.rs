pub mod battle;
pub mod time_measurer;
pub mod random_getter;
pub mod pvs_getter;

pub mod staged_rater;
pub mod combined_rater;

pub mod fish_difference_rater;
pub mod potential_fish_rater;
pub mod reachable_fish_rater;
pub mod penguin_restrictions;
pub mod bitset_penguin_restrictions;
pub mod vec_penguin_restrictions;
pub mod restricted_reachable_fish_rater;
pub mod quadrant_occupation_rater;
pub mod penguin_cutoff_rater;

use rostware23_lib::game::state::State;
use rostware23_lib::game::moves::Move;

use time_measurer::TimeMeasurer;

pub trait MoveGetter {
    fn get_move(&self, state: &State, time_measurer: &TimeMeasurer) -> anyhow::Result<Move>;
}

pub trait Rater {
    fn rate(state: &State) -> i32;
}
