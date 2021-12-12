use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

mod decode;
mod encode;
use decode::*;
use encode::*;

pub fn bench_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encoder");
    for i in [1024 * 1024 * 4, 1024 * 1024 * 32] {
        group
            .throughput(Throughput::Bytes(i as u64)) // show throughput
            .bench_with_input(BenchmarkId::new("BasicEncoder", i), &i, basic_encoder)
            .bench_with_input(BenchmarkId::new("FastEncoder", i), &i, fast_encoder)
            .bench_with_input(BenchmarkId::new("BasicDecoder", i), &i, basic_decoder);
    }
    for i in [1024 * 1024 * 128] {
        group
            .throughput(Throughput::Bytes(i as u64))
            .sample_size(50)
            .measurement_time(std::time::Duration::from_secs(10))
            .bench_with_input(BenchmarkId::new("BasicEncoder", i), &i, basic_encoder)
            .bench_with_input(BenchmarkId::new("FastEncoder", i), &i, fast_encoder)
            .bench_with_input(BenchmarkId::new("BasicDecoder", i), &i, basic_decoder);
    }
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
