use criterion::{Criterion, criterion_group, criterion_main};

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

fn sha256_single_file() {
    let _result = file_sha256(&FILES[0]);
}

fn blake3_single_file() {
    let _result = file_blake3(&FILES[0]);
}

fn sha256_seq_for_loop() {
    let _results = seq_for_loop(&FILES, Hash::Sha256);
}

fn sha256_seq_iter() {
    let _results = seq_iter(&FILES, Hash::Sha256);
}

fn sha256_threading() {
    let _results = threading(&FILES, Hash::Sha256);
}

fn sha256_messaging() {
    let _results = messaging(&FILES, Hash::Sha256);
}

fn sha256_rayon_par_iter() {
    let _results = rayon_par_iter(&FILES, Hash::Sha256);
}

fn blake3_seq_for_loop() {
    let _results = seq_for_loop(&FILES, Hash::Blake3);
}

fn blake3_seq_iter() {
    let _results = seq_iter(&FILES, Hash::Blake3);
}

fn blake3_threading() {
    let _results = threading(&FILES, Hash::Blake3);
}

fn blake3_messaging() {
    let _results = messaging(&FILES, Hash::Blake3);
}

fn blake3_rayon_par_iter() {
    let _results = rayon_par_iter(&FILES, Hash::Blake3);
}

fn bench(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("SingleFile");
        group.bench_function("Sha256", |b| b.iter(sha256_single_file));
        group.bench_function("Blake3", |b| b.iter(blake3_single_file));
    }

    {
        let mut group = c.benchmark_group("ProcessOption");
        group.bench_function("SequentialForLoop/Sha256", |b| b.iter(sha256_seq_for_loop));
        group.bench_function("SequentialIter/Sha256", |b| b.iter(sha256_seq_iter));
        group.bench_function("Threading/Sha256", |b| b.iter(sha256_threading));
        group.bench_function("Messaging/Sha256", |b| b.iter(sha256_messaging));
        group.bench_function("RayonParIter/Sha256", |b| b.iter(sha256_rayon_par_iter));
        group.bench_function("SequentialForLoop/Blake3", |b| b.iter(blake3_seq_for_loop));
        group.bench_function("SequentialIter/Blake3", |b| b.iter(blake3_seq_iter));
        group.bench_function("Threading/Blake3", |b| b.iter(blake3_threading));
        group.bench_function("Messaging/Blake3", |b| b.iter(blake3_messaging));
        group.bench_function("RayonParIter/Blake3", |b| b.iter(blake3_rayon_par_iter));
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
