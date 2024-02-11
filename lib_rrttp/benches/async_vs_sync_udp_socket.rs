use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_vs_sync_udp_socket");
    group.sample_size(10);
    //   group.bench_function("sync", |b| b.iter(test_sync_udp_socket));
    //  group.bench_function("async", |b| b.iter(test_async_udp_socket));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
