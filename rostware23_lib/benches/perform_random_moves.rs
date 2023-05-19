extern crate rostware23_lib;
extern crate criterion;

use anyhow::Context;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::prelude::*;

use rostware23_lib::game::moves::*;
use rostware23_lib::game::state::*;
use rostware23_lib::game::state_generator::*;

fn advance_state_randomly(state: State) -> anyhow::Result<State> {
    let possible_moves_iter = state.possible_moves();
    let possible_moves: Vec<Move> = possible_moves_iter.collect();
    let chosen_move = possible_moves.choose(&mut rand::thread_rng()).context("No possible moves")?;
    state.with_move_performed(chosen_move.clone())
}

fn perform_some_random_moves() -> anyhow::Result<()> {
    let mut state = create_any(); 
    while !state.is_over() { 
        state = advance_state_randomly(state)?;
    }
    Ok(())
}

pub fn perform_random_moves_on_any_state_benchmark(c: &mut Criterion) {
    c.bench_function("perform random moves", |b| b.iter(|| black_box(
        perform_some_random_moves()
    )));
}

criterion_group!(benches, perform_random_moves_on_any_state_benchmark);
criterion_main!(benches);
