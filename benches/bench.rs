use criterion::{criterion_group, criterion_main, Criterion};

use fhc::*;

const FILES: [&str; 11] = [
    "benches/bench.rs",
    "Cargo.toml",
    "CHANGELOG.md",
    "Makefile.md",
    "README.md",
    "src/bin/fhc.rs",
    "src/lib.rs",
    "src/tests.rs",
    "tests/empty.txt",
    "tests/lorem.txt",
    "t/README.md",
];

fn single_file() {
    let _result = sha256(&FILES[0]);
}

fn _seq_for_loop() {
    let _results = seq_for_loop(&FILES);
}

fn _seq_iter() {
    let _results = seq_iter(&FILES);
}

fn _threading() {
    let _results = threading(&FILES);
}

fn _messaging() {
    let _results = messaging(&FILES);
}

fn _rayon_par_iter() {
    let _results = rayon_par_iter(&FILES);
}

fn bench(c: &mut Criterion) {
    c.bench_function("Single file", |b| b.iter(single_file));

    let mut group = c.benchmark_group("ProcessOption");
    group.bench_function("SequentialForLoop", |b| b.iter(_seq_for_loop));
    group.bench_function("SequentialIter", |b| b.iter(_seq_iter));
    group.bench_function("Threading", |b| b.iter(_threading));
    group.bench_function("Messaging", |b| b.iter(_messaging));
    group.bench_function("RayonParIter", |b| b.iter(_rayon_par_iter));
}

criterion_group!(benches, bench);
criterion_main!(benches);
