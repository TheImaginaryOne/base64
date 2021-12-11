use base64::basic::*;
use base64::interface::*;
use criterion::{black_box, Bencher};

use rand::{rngs::SmallRng, RngCore, SeedableRng};

fn random_base64(len: usize) -> Vec<u8> {
    let mut input = vec![0; (len / 4) * 3];
    let mut rng = SmallRng::from_seed(*b"behejw##&*sbry219lkmzlk;-=[[3;nd");
    rng.fill_bytes(&mut input[..]);

    let mut encoded = vec![0; len];
    let encoder = BasicEncoder::new();
    encoder.encode(&input, &mut encoded);
    encoded
}

pub fn basic_decoder(b: &mut Bencher, &size: &usize) {
    let input = random_base64(size);
    let mut output = vec![0; (size / 4) * 3];
    let decoder = BasicDecoder::new();
    b.iter(|| decoder.decode(&input, black_box(&mut output)));
}

