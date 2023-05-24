use std::marker::PhantomData;

use rostware23_lib::game::state::State;

use super::Rater;

const EARLY_GAME_MAX_TURN: u32 = 8;

pub struct StagedRater<
    const EARLY_MULTIPLIER: i32,
    const MID_MULTIPLIER: i32,
    const END_MULTIPLIER: i32,
    Heuristic: Rater,
> {
    phantom: PhantomData<Heuristic>,
}

impl<
        const EARLY_MULTIPLIER: i32,
        const MID_MULTIPLIER: i32,
        const END_MULTIPLIER: i32,
        Heuristic: Rater,
    > Rater for StagedRater<EARLY_MULTIPLIER, MID_MULTIPLIER, END_MULTIPLIER, Heuristic>
{
    fn rate(state: &State) -> i32 {
        if state.turn < EARLY_GAME_MAX_TURN {
            return EARLY_MULTIPLIER * Heuristic::rate(state);
        }

        let opponent_team = state.current_team().opponent();
        if !state.has_team_any_moves(opponent_team) {
            return END_MULTIPLIER * Heuristic::rate(state);
        }

        MID_MULTIPLIER * Heuristic::rate(state)
    }
}
