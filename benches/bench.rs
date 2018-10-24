#[macro_use]
extern crate criterion;
extern crate haml;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let haml = include_str!("inputs/01_basic.haml");
    c.bench_function("01 Basic", move |b| b.iter(|| haml::to_html(haml)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
