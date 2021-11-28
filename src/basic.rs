use crate::traits::Base64Encoder;

pub struct BasicEncoder {}

impl BasicEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const UNROLL_SIZE: usize = 8;

#[inline(always)]
fn encode_one(i: u8) -> u8 {
    return BASE64[i as usize];
}

#[inline(always)]
fn decode_chunk(input: &[u8], output: &mut [u8]) {
    let (a, b, c) = (input[0], input[1], input[2]);

    output[0] = encode_one(a >> 2);
    output[1] = encode_one(((3 & a) << 4) | (b >> 4));
    output[2] = encode_one(((15 & b) << 2) | (c >> 6));
    output[3] = encode_one(c & 63);
}

impl Base64Encoder for BasicEncoder {
    fn encode(&self, input: &[u8], output: &mut [u8]) {
        let n_groups = input.len() / 3;
        let n_remainder = input.len() % 3;

        // manual unroll?
        let mut i = 0;
        while i + UNROLL_SIZE < n_groups {
            let chunk = &input[3 * i..3 * (i + UNROLL_SIZE)];
            let chunk_out = &mut output[4 * i..4 * (i + UNROLL_SIZE)];
            for j in 0..UNROLL_SIZE {
                decode_chunk(&chunk[3 * j..3 * (j + 1)], &mut chunk_out[4 * j..4 * (j + 1)]);
            }

            i += UNROLL_SIZE;
        }
        for i in ((n_groups / UNROLL_SIZE) * UNROLL_SIZE)..n_groups {
            decode_chunk(&input[3 * i..3 * (i + 1)], &mut output[4 * i..4 * (i + 1)]);
        };

        let remain_start = 4 * n_groups;
        // padding
        if n_remainder == 1 {
            let a = input[3 * n_groups];
            output[remain_start] = encode_one(a >> 2);
            output[remain_start + 1] = encode_one((3 & a) << 4);
            output[remain_start + 2] = '=' as u8;
            output[remain_start + 3] = '=' as u8;
        } else if n_remainder == 2 {
            let a = input[3 * n_groups];
            let b = input[3 * n_groups + 1];
            output[remain_start] = encode_one(a >> 2);
            output[remain_start + 1] = encode_one(((3 & a) << 4) | (b >> 4));
            output[remain_start + 2] = encode_one((15 & b) << 2);
            output[remain_start + 3] = '=' as u8;
        }
    }
}
