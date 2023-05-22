extern crate rand;

use rostware23_lib::game::moves::Move;
use rostware23_lib::game::state::State;

use super::MoveGetter;
use super::time_measurer::TimeMeasurer;
use anyhow::Context;
use rand::seq::SliceRandom;

pub struct RandomGetter {
}

impl RandomGetter {
    pub fn new() -> Self {
        Self {}
    }
}

impl MoveGetter for RandomGetter {
    fn get_move(&self, state: &State, time_measurer: &TimeMeasurer) -> anyhow::Result<Move> {
        let possible_moves_iter = state.possible_moves();
        let possible_moves: Vec<Move> = possible_moves_iter.collect();
        let chosen_move = possible_moves.choose(&mut rand::thread_rng()).context("No possible moves found")?;
        Ok(chosen_move.clone())
    }
}
