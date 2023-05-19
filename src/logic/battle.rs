use rostware23_lib::game::result::GameResult;
use rostware23_lib::game::state::State;
use rostware23_lib::game::state_generator::create_any;
use rostware23_lib::xml::common::Team;

use super::MoveGetter;

#[derive(Debug)]
pub struct BattleOutcome(u32, u32);

impl BattleOutcome {
    fn empty() -> Self {
        Self(0, 0)
    }

    fn from_results(results: Vec<GameResult>) -> Self {
        let mut win_amount_team_one = 0;
        let mut win_amount_team_two = 0;

        for result in results {
            if let Some(winner) = result.winner {
                match winner {
                    Team::One => win_amount_team_one += 1,
                    Team::Two => win_amount_team_two += 1,
                }
            }
        }

        Self(win_amount_team_one, win_amount_team_two)
    }

    fn combined(outcomes: Vec<BattleOutcome>) -> Self {
        outcomes.into_iter().fold(Self::empty(), |first, second| {
            Self(first.0 + second.0, first.1 + second.1)
        })
    }

    pub fn winner(&self) -> Option<Team> {
        if self.0 > self.1 { Some(Team::One) }
        else if self.1 > self.0 { Some(Team::Two) }
        else { None }
    }
}

pub struct Battle<'playout> {
    getter_team_one: &'playout dyn MoveGetter,
    getter_team_two: &'playout dyn MoveGetter
}

impl<'playout> Battle<'playout> {
    pub fn between(getter_team_one: &'playout dyn MoveGetter, getter_team_two: &'playout dyn MoveGetter) -> Self {
        Self { getter_team_one, getter_team_two }
    }

    fn move_getter_for_team(&'playout self, team: Team) -> &'playout dyn MoveGetter {
        match team {
            Team::One => self.getter_team_one,
            Team::Two => self.getter_team_two,
        }
    }

    pub fn mono_directional_with_start_team(&self, mut state: State, mut current_team: Team) -> anyhow::Result<BattleOutcome> {
        while !state.is_over() {
            let current_getter = self.move_getter_for_team(current_team.clone());
            let performed_move = current_getter.get_move(&state);
            if performed_move.is_err() {
                state = state.with_moveless_player_skipped()?;
            } else {
                state.perform_move(performed_move.unwrap())?;
            }
            current_team = current_team.opponent();
        }
        let result = state.get_result()?;

        Ok(BattleOutcome::from_results(vec![result]))
    }

    pub fn bi_directional(&self, state: State) -> anyhow::Result<BattleOutcome> {
        let first_result = self.mono_directional_with_start_team(state.clone(), Team::One)?;
        let second_result = self.mono_directional_with_start_team(state, Team::Two)?;
        Ok(BattleOutcome::combined(vec![
            first_result,
            second_result
        ]))
    }

    pub fn multiple_bi_directional(&self, amount: usize) -> anyhow::Result<BattleOutcome> {
        let mut outcomes = vec![];
        for _ in 0..amount {
            outcomes.push(self.bi_directional(create_any())?);
        }
        Ok(BattleOutcome::combined(outcomes))
    }
}

