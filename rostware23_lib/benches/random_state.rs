extern crate rostware23_lib;
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rostware23_lib::game::state_generator::*;

pub fn create_state_benchmark(c: &mut Criterion) {
    c.bench_function("create random state", |b| b.iter(|| black_box(create_any())));
}

criterion_group!(benches, create_state_benchmark);
criterion_main!(benches);
