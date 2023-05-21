use rostware23_lib::game::state::State;
use super::Rater;
use super::fish_difference_rater::FishDifferenceRater;
use super::potential_fish_rater::PotentialFishRater;

pub struct CombinedRater {}

impl Rater for CombinedRater {
    fn rate(state: &State) -> i32 {
        5 * FishDifferenceRater::rate(state) + PotentialFishRater::rate(state)
    }
}
