extern crate rostware23_lib;
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rostware23_lib::game::state_generator::*;

pub fn possible_moves_from_any_state_benchmark(c: &mut Criterion) {
    c.bench_function("current team", |b| b.iter(|| black_box(create_any().current_team())));
}

criterion_group!(benches, possible_moves_from_any_state_benchmark);
criterion_main!(benches);
