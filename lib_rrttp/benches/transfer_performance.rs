use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("network_transfer");
    group.bench_function("udp transfer", |b| b.iter(udp));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
