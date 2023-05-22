use rostware23_lib::game::state::State;
use super::Rater;
use super::fish_difference_rater::FishDifferenceRater;
use super::potential_fish_rater::PotentialFishRater;
use super::reachable_fish_rater::ReachableFishRater;
use super::quadrant_occupation_rater::QuadrantOccupationRater;
use super::penguin_cutoff_rater::PenguinCutOffRater;

pub struct CombinedRater {}

impl Rater for CombinedRater {
    fn rate(state: &State) -> i32 {
        20 * FishDifferenceRater::rate(state) +
            2 * PotentialFishRater::rate(state) +
            2 * PenguinCutOffRater::rate(state) +
            5 * ReachableFishRater::rate(state) +
            10 * QuadrantOccupationRater::rate(state)
    }
}
