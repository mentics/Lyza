use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

use std::collections::{HashMap,BTreeMap};

fn bench_hash_map(keys: &Vec<i64>) -> i64 {
    let mut m = HashMap::new();
    for i in keys {
        m.insert(i, 1000);
    }
    let mut s = 0;
    for i in keys {
        s += m.get(&i).unwrap()
    }
    return s;
}

fn bench_btree_map(keys: &Vec<i64>) -> i64 {
    let mut m = BTreeMap::new();
    for i in keys {
        m.insert(i, 1000);
    }
    let mut s = 0;
    for i in keys {
        s += m.get(&i).unwrap()
    }
    return s;
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut keys: Vec<i64> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..200 {
        keys.push(rng.gen());
    }
    c.bench_function("HashMap", |b| b.iter(|| bench_hash_map(black_box(&keys))));
    c.bench_function("BTreeMap", |b| b.iter(|| bench_btree_map(black_box(&keys))));
    c.bench_function("HashMap2", |b| b.iter(|| bench_hash_map(black_box(&keys))));
    c.bench_function("BTreeMap2", |b| b.iter(|| bench_btree_map(black_box(&keys))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);