use rostware23_lib::game::state::State;
use crate::logic::bitset_penguin_restrictions::BitsetPenguinRestrictions;
use crate::logic::restricted_reachable_fish_rater::RestrictedReachableFishRater;
use crate::logic::vec_penguin_restrictions::VecPenguinRestrictions;
use super::Rater;
use super::edge_penguin_penalty::EdgePenguinPenalty;
use super::fish_difference_rater::FishDifferenceRater;
use super::potential_fish_rater::PotentialFishRater;
use super::quadrant_occupation_rater::QuadrantOccupationRater;
use super::penguin_cutoff_rater::PenguinCutOffRater;
use super::staged_rater::StagedRater;

pub struct CombinedRater {}

impl Rater for CombinedRater {
    fn rate(state: &State) -> i32 {
            20 *FishDifferenceRater::rate(state) +
            2 * PenguinCutOffRater::rate(state) +
            10 * QuadrantOccupationRater::rate(state) +
            5 * RestrictedReachableFishRater::<VecPenguinRestrictions>::rate(state) +
            StagedRater::<1, 0, 0, EdgePenguinPenalty>::rate(state) +
            3 * PotentialFishRater::rate(state)
    }
}
