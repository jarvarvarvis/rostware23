extern crate criterion;
extern crate rostware23_lib;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rostware23_lib::game::state_generator::*;
use rostware23_lib::xml::common::Team;

pub fn has_team_any_moves_on_random_state_benchmark(c: &mut Criterion) {
    c.bench_function("has team any moves on random state", |b| {
        b.iter(|| black_box(create_any().has_team_any_moves(Team::One)))
    });
}

criterion_group!(benches, has_team_any_moves_on_random_state_benchmark);
criterion_main!(benches);
