// Benchmarking the network module.
use criterion::{criterion_group, criterion_main, Criterion};
// Assuming 'simple_rust' is the correct crate name in your Cargo.toml.
use simple_rust::network;

fn network_benchmark(c: &mut Criterion) {
    c.bench_function("network::connect", |b| b.iter(|| network::connect()));
}

criterion_group!(benches, network_benchmark);
criterion_main!(benches);
