use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tgm_acc_2019_expert_level3::rayon::m;

pub fn bench_m_10(criterion: &mut Criterion) {
    criterion.bench_function("bench m(10)", |b| b.iter(|| m(black_box(10))));
}

pub fn bench_m_10_000(criterion: &mut Criterion) {
    criterion.bench_function("bench m(10^4)", |b| b.iter(|| m(black_box(10_000))));
}

criterion_group!(benches, bench_m_10, bench_m_10_000);
criterion_main!(benches);
