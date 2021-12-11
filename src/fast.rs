use crate::interface::Base64Encoder;
use crate::utils::encode_remainder;
use core::arch::x86_64::*;

pub struct FastEncoder {}

impl FastEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

#[inline(always)]
unsafe fn sse_encode_chunk(input: &[u8], output: &mut [u8]) {
    // Load 12 bytes
    let mut data = _mm_loadu_si128(input.as_ptr() as *const __m128i);
    // We only consider the first 12 bytes, and we redistribute the data to have 16 bytes
    // of 6 bit data.

    // Step 1:
    // we go from
    // _,_,_,_,z2,z1,z0,y2,y1,y0,x2,x1,x0,w2,w1,w0
    // to
    // z1,z2,z0,z1, .... ,w1,w2,w0,w1
    // (_ are ignored data)
    // Note Intel is little-endian.
    data = _mm_shuffle_epi8(
        data,
        _mm_set_epi8(10, 11, 9, 10, 7, 8, 6, 7, 4, 5, 3, 4, 1, 2, 0, 1),
    );
    // Move groups of 6 bits to the correct places.
    // Bitmask on each block of 4 bytes:
    // 0000c1..c4/c5,c6 00000/a1..a6 00/00000000
    // Note set1 means to broadcast on all 16 bytes.
    let mut a = _mm_and_si128(data, _mm_set1_epi32(0x0fc0fc00));
    // Bit shift with multiplications on 2 byte lanes
    // mulhi means multiply 16 bit ints and store the high 16 bits of the result
    a = _mm_mulhi_epu16(a, _mm_set1_epi32(0x04000040));

    // 00000000/00 d1..d6/000000b1b2/b3..b4 0000
    let mut b = _mm_and_si128(data, _mm_set1_epi32(0x003f03f0));
    b = _mm_mullo_epi16(b, _mm_set1_epi32(0x01000010));

    let splitted = _mm_or_si128(a, b);

    // --- Lookup stage ---

    // Saturating sub.
    // 0..51  -> 0
    // 52..63 -> 1..12
    let reduced = _mm_subs_epu8(splitted, _mm_set1_epi8(51));

    // 0..25  -> 11111111
    // 26..51 -> 00000000
    let cmp = _mm_cmpgt_epi8(_mm_set1_epi8(26), splitted);
    // 26..51 -> 0 (dec)
    // 0..25 -> 13 (dec)
    let lowercase = _mm_and_si128(cmp, _mm_set1_epi8(13));

    let indices = _mm_or_si128(reduced, lowercase);
    let y = '0' as i8 - 52;
    let lookup_table = _mm_setr_epi8(
        'a' as i8 - 26,
        y,
        y,
        y,
        y,
        y,
        y,
        y,
        y,
        y,
        y,
        '+' as i8 - 62,
        '/' as i8 - 63,
        'A' as i8,
        0,
        0,
    );

    let offset = _mm_shuffle_epi8(lookup_table, indices);
    // add offset to result to get final encoding
    let result = _mm_add_epi8(offset, splitted);

    // store 16 bytes of results
    _mm_storeu_si128(output.as_mut_ptr() as *mut __m128i, result);
}

#[target_feature(enable = "ssse3")]
unsafe fn sse_encode(input: &[u8], output: &mut [u8]) {
    const UNROLL_SIZE: usize = 8;
    let n_groups = input.len() / 12;

    let mut i = 0;
    // We don't want to read 12 bytes using a _mm_loadu_si128
    // which actually reads 16 bytes and overflows the vector.
    while i + UNROLL_SIZE + 1 <= n_groups {
        let chunk = &input[12 * i..12 * (i + UNROLL_SIZE)];
        let chunk_out = &mut output[16 * i..16 * (i + UNROLL_SIZE)];
        for j in 0..UNROLL_SIZE {
            // 12 bytes input; 16 bytes output
            sse_encode_chunk(
                &chunk[12 * j..12 * (j + 1)],
                &mut chunk_out[16 * j..16 * (j + 1)],
            );
        }
        i += UNROLL_SIZE;
    }
    let remaining_start = (n_groups.saturating_sub(1) / UNROLL_SIZE) * UNROLL_SIZE;
    encode_remainder(
        &input[12 * remaining_start..],
        &mut output[16 * remaining_start..],
    );
}

impl Base64Encoder for FastEncoder {
    fn encode(&self, input: &[u8], output: &mut [u8]) {
        // Undefined behaviour if SSE4.1 is not supported
        unsafe {
            sse_encode(input, output);
        }
    }
}
