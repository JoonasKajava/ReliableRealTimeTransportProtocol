use std::fs;

use async_std::task::block_on;
use criterion::{criterion_group, criterion_main, Criterion};
use futures::stream::FuturesUnordered;
use futures::StreamExt;

use lib_rrttp::transport_layer::constants::MAX_DATA_SIZE;
use lib_rrttp::transport_layer::socket::SocketAbstraction;

fn test_sync_udp_socket() {
    block_on(async {
        let socket = SocketAbstraction::bind("localhost:12345").await.unwrap();
        socket.connect("localhost:12345").await.unwrap();
        let vec = fs::read("D:\\Videos\\2023-03-17 18-15-38.mp4").unwrap();

        let chunks = vec.chunks(MAX_DATA_SIZE);
        for chunk in chunks {
            socket.send(chunk).await.unwrap();
        }
    });
}

fn test_async_udp_socket() {
    block_on(async {
        let socket = SocketAbstraction::bind("localhost:12345").await.unwrap();
        socket.connect("localhost:12345").await.unwrap();

        let vec = fs::read("D:\\Videos\\2023-03-17 18-15-38.mp4").unwrap();
        let futures = FuturesUnordered::new();

        let chunks = vec.chunks(MAX_DATA_SIZE);
        for chunk in chunks {
            futures.push(socket.send(chunk));
        }
        let results: Vec<_> = futures.collect().await;
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_vs_sync_udp_socket");
    group.sample_size(10);
    group.bench_function("sync", |b| b.iter(test_sync_udp_socket));
    group.bench_function("async", |b| b.iter(test_async_udp_socket));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
