use crate::traits::{Base64Encoder, Base64Decoder};
use crate::utils::{encode_remainder, encode_three_byte_chunk};

pub struct BasicEncoder {}

impl BasicEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

const UNROLL_SIZE: usize = 8;

impl Base64Encoder for BasicEncoder {
    fn encode(&self, input: &[u8], output: &mut [u8]) {
        let n_groups = input.len() / 3;

        // manual unroll?
        let mut i = 0;
        while i + UNROLL_SIZE <= n_groups {
            let chunk = &input[3 * i..3 * (i + UNROLL_SIZE)];
            let chunk_out = &mut output[4 * i..4 * (i + UNROLL_SIZE)];
            for j in 0..UNROLL_SIZE {
                encode_three_byte_chunk(
                    &chunk[3 * j..3 * (j + 1)],
                    &mut chunk_out[4 * j..4 * (j + 1)],
                );
            }

            i += UNROLL_SIZE;
        }

        let remaining_start = (n_groups / UNROLL_SIZE) * UNROLL_SIZE;
        encode_remainder(
            &input[3 * remaining_start..],
            &mut output[4 * remaining_start..],
        );
    }
}

pub struct BasicDecoder {}

impl BasicDecoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Base64Decoder for BasicDecoder {
    fn decode(&self, input: &[u8], output: &mut [u8]) -> Result<(), ()> {
        if input.len() % 4 > 0 {
            return Err(())
        }
        let n_groups = input.len() / 4;
        Ok(())
    }
}

