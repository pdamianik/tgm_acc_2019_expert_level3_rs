use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tgm_acc_2019_expert_level3::initial::m;

pub fn bench_m_10(criterion: &mut Criterion) {
    criterion.bench_function("bench m(10)", |b| b.iter(|| m(black_box(10))));
}

criterion_group!(benches, bench_m_10);
criterion_main!(benches);
