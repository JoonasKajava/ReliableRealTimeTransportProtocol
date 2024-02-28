use std::collections::{LinkedList, VecDeque};

use criterion::{criterion_group, criterion_main, Criterion};

const WINDOW_SIZE: usize = 100000;
const shift_amount: usize = 100;

fn bench_array_shifting() -> Vec<usize> {
    let mut array = [0; WINDOW_SIZE];
    for i in 0..WINDOW_SIZE {
        array[i] = i;
    }
    let result = array[0..shift_amount].to_vec();
    array.rotate_left(shift_amount);
    result
}

fn bench_boxed_array_shifting() -> Vec<usize> {
    let mut array = Box::new([0; WINDOW_SIZE]);
    for i in 0..WINDOW_SIZE {
        array[i] = i;
    }
    let result = array[0..shift_amount].to_vec();
    array.rotate_left(shift_amount);
    result
}

fn bench_vec_shifting() -> Vec<usize> {
    let mut vec = Vec::with_capacity(WINDOW_SIZE);
    for i in 0..WINDOW_SIZE {
        vec.push(i);
    }

    vec.drain(0..shift_amount).collect()
}

fn bench_vecdeque_shifting() -> Vec<usize> {
    let mut vec = VecDeque::with_capacity(WINDOW_SIZE);
    for i in 0..WINDOW_SIZE {
        vec.push_back(i);
    }

    vec.drain(0..shift_amount).collect()
}

fn bench_linked_list_shifting() -> Vec<usize> {
    let mut linked_list = LinkedList::new();
    for i in 0..WINDOW_SIZE {
        linked_list.push_back(i);
    }
    let mut vec = vec![];
    for _ in 0..shift_amount {
        vec.push(linked_list.pop_front().unwrap());
    }
    vec
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections");
    group.bench_function("vec shifting", |b| b.iter(bench_vec_shifting));
    group.bench_function("linked list shifting", |b| {
        b.iter(bench_linked_list_shifting)
    });
    group.bench_function("vecdeque shifting", |b| b.iter(bench_vecdeque_shifting));
    group.bench_function("array shifting", |b| b.iter(bench_array_shifting));
    group.bench_function("boxed array shifting", |b| {
        b.iter(bench_boxed_array_shifting)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
