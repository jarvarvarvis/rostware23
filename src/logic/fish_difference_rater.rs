use rostware23_lib::game::state::State;
use super::Rater;

pub struct FishDifferenceRater {}

impl Rater for FishDifferenceRater {
    fn rate(game_state: &State) -> i32 {
        game_state.score_of_team(game_state.current_team()) as i32 - game_state.score_of_team(game_state.current_team().opponent()) as i32
    }
}
