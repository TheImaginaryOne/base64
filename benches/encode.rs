use base64::basic::BasicEncoder;
use base64::fast::FastEncoder;
use base64::traits::Base64Encoder;
use criterion::{
    black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion, Throughput,
};
use rand::{rngs::SmallRng, RngCore, SeedableRng};

fn random_bytes(len: usize) -> Vec<u8> {
    let mut vec = vec![0; len];
    let mut rng = SmallRng::from_seed(*b"behejw##&*sbry219lkmzlk;-=[[3;nd");
    rng.fill_bytes(&mut vec[..]);
    vec
}

// There are benchmarks for basic encoder and (TODO) fast encoder.

pub fn basic_encoder(b: &mut Bencher, &size: &usize) {
    let input = random_bytes(size);
    let mut output = vec![0; 4 * (1 + size / 3)];
    let encoder = BasicEncoder::new();
    b.iter(|| encoder.encode(&input, black_box(&mut output)));
}

pub fn fast_encoder(b: &mut Bencher, &size: &usize) {
    let input = random_bytes(size);
    let mut output = vec![0; 4 * (1 + size / 3)];
    let encoder = FastEncoder::new();
    b.iter(|| encoder.encode(&input, black_box(&mut output)));
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encoder");
    for i in [1024, 1024 * 256, 1024 * 1024 * 4, 1024 * 1024 * 32] {
        group
            .throughput(Throughput::Bytes(i as u64)) // show throughput
            .bench_with_input(BenchmarkId::new("Basic", i), &i, basic_encoder);
        group
            .throughput(Throughput::Bytes(i as u64)) // show throughput
            .bench_with_input(BenchmarkId::new("Fast", i), &i, fast_encoder);
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
