use base64::basic::BasicEncoder;
use base64::fast::FastEncoder;
use base64::interface::Base64Encoder;
use criterion::{
    black_box, Bencher
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
