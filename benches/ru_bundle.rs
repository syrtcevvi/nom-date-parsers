use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use nom_date_parsers::i18n::ru::bundle;

fn ru_bundle_benchmark(c: &mut Criterion) {
    c.bench_function("ru bundle", |b| b.iter(|| bundle(black_box("Воскресенье"))));
}

criterion_group!(benches, ru_bundle_benchmark);
criterion_main!(benches);
