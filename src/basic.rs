use crate::traits::Base64Encoder;

pub struct BasicEncoder {}

impl BasicEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

fn encode_one(i: u8) -> u8 {
    if i < 26 {
        // range A-Z
        i + 'A' as u8
    } else if i >= 26 && i < 52 {
        // range a-z
        i + 'a' as u8 - 26
    } else if i >= 52 && i < 62 {
        // range 0-9
        i + '0' as u8 - 52
    } else if i == 62 {
        // character plus
        '+' as u8
    } else if i == 63 {
        // character slash
        '/' as u8
    } else {
        unreachable!("Can only have 6 bits");
    }
}

impl Base64Encoder for BasicEncoder {
    fn encode(&self, input: &[u8], output: &mut [u8]) {
        let n_groups = input.len() / 3;
        let n_remainder = input.len() % 3;

        for i in 0..n_groups {
            let (a, b, c) = (input[3 * i], input[3 * i + 1], input[3 * i + 2]);
            let start = 4 * i;

            output[start] = encode_one(a >> 2);
            output[start + 1] = encode_one(((3 & a) << 4) | (b >> 4));
            output[start + 2] = encode_one(((15 & b) << 2) | (c >> 6));
            output[start + 3] = encode_one(c & 63);
        }
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
