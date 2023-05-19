extern crate rostware23_lib;
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rostware23_lib::game::moves::Move;
use rostware23_lib::game::state_generator::*;
use rostware23_lib::game::possible_moves::PossibleMovesIterator;
use rostware23_lib::xml::common::Team;

pub fn team_one_possible_moves_from_any_state_benchmark(c: &mut Criterion) {
    c.bench_function("get team one possible moves", |b| b.iter(|| black_box(
                PossibleMovesIterator::from_state_and_team(create_any(), Team::One).collect::<Vec<Move>>()
    )));
}

criterion_group!(benches, team_one_possible_moves_from_any_state_benchmark);
criterion_main!(benches);
