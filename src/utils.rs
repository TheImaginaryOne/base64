const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[inline(always)]
pub fn encode_one(i: u8) -> u8 {
    return BASE64[i as usize];
}

#[inline(always)]
pub fn encode_three_byte_chunk(input: &[u8], output: &mut [u8]) {
    let (a, b, c) = (input[0], input[1], input[2]);

    output[0] = encode_one(a >> 2);
    output[1] = encode_one(((3 & a) << 4) | (b >> 4));
    output[2] = encode_one(((15 & b) << 2) | (c >> 6));
    output[3] = encode_one(c & 63);
}

#[inline]
pub fn encode_remainder(remainder_input: &[u8], remainder_output: &mut [u8]) {
    let n_remaining_groups = remainder_input.len() / 3;
    for i in 0..n_remaining_groups {
        encode_three_byte_chunk(
            &remainder_input[3 * i..3 * (i + 1)],
            &mut remainder_output[4 * i..4 * (i + 1)],
        );
    }

    // The final bytes which might have padding.
    let input_tail = &remainder_input[3 * n_remaining_groups..];
    let output_tail = &mut remainder_output[4 * n_remaining_groups..];

    let n_remainder = input_tail.len();
    if n_remainder == 0 {
        return;
    } else if n_remainder == 1 {
        let a = input_tail[0];
        output_tail[0] = encode_one(a >> 2);
        output_tail[1] = encode_one((3 & a) << 4);
        output_tail[2] = '=' as u8;
        output_tail[3] = '=' as u8;
    } else
    /* if n_remainder == 2 */
    {
        let a = input_tail[0];
        let b = input_tail[1];
        output_tail[0] = encode_one(a >> 2);
        output_tail[1] = encode_one(((3 & a) << 4) | (b >> 4));
        output_tail[2] = encode_one((15 & b) << 2);
        output_tail[3] = '=' as u8;
    }
}
