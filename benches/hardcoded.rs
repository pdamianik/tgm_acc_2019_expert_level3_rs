use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tgm_acc_2019_expert_level3::hardcoded::m;

pub fn bench_m_10(criterion: &mut Criterion) {
    criterion.bench_function("bench m(10)", |b| b.iter(|| m(black_box(10))));
}

pub fn bench_m_10_000(criterion: &mut Criterion) {
    criterion.bench_function("bench m(10^4)", |b| b.iter(|| m(black_box(10_000))));
}

pub fn bench_m_2_000_000_000(criterion: &mut Criterion) {
    criterion.bench_function("bench m(2*10^9)", |b| b.iter(|| m(black_box(2_000_000_000))));
}

pub fn bench_m_u128_max(criterion: &mut Criterion) {
    criterion.bench_function("bench m(340282366920938463463374607431768211455)", |b| b.iter(|| m(black_box(340282366920938463463374607431768211455))));
}

criterion_group!(benches, bench_m_10, bench_m_10_000, bench_m_2_000_000_000, bench_m_u128_max);
criterion_main!(benches);
