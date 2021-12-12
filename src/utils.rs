const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const INVALID_VALUE: u8 = 0xff;
const BASE64_DECODE: &[u8] = &generate_decode_table(BASE64);

const fn generate_decode_table(encode_table: &[u8]) -> [u8; 256] {
    let mut decode_table = [INVALID_VALUE; 256];
    // Rust const fn does not support for loops
    let mut i: u8 = 0;
    while i < 64 {
        let encoded = encode_table[i as usize];
        decode_table[encoded as usize] = i;
        i += 1;
    }
    decode_table
}

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

/// Decode a byte to 6 bits.
#[inline(always)]
fn decode_one(i: u8) -> Result<u8, ()> {
    let x = BASE64_DECODE[i as usize];
    if x < 255 {
        Ok(x)
    } else {
        Err(())
    }
}

#[inline(always)]
pub fn decode_four_byte_chunk(input: &[u8], output: &mut [u8]) -> Result<(), usize> {
    let (a, b, c, d) = (
        decode_one(input[0]).map_err(|_| 0usize)?,
        decode_one(input[1]).map_err(|_| 1usize)?,
        decode_one(input[2]).map_err(|_| 2usize)?,
        decode_one(input[3]).map_err(|_| 3usize)?,
    );

    output[0] = (a << 2) | (b >> 4);
    output[1] = (b << 4) | (c >> 2);
    output[2] = (c << 6) | d;
    Ok(())
}

#[inline]
pub fn decode_remainder(remainder_input: &[u8], remainder_output: &mut [u8]) -> Result<(), usize> {
    if remainder_input.len() < 4 {
        return Ok(())
    }

    let n_remaining_groups = remainder_input.len() / 4;
    for i in 0..n_remaining_groups - 1 {
        decode_four_byte_chunk(
            &remainder_input[4 * i..4 * (i + 1)],
            &mut remainder_output[3 * i..3 * (i + 1)],
        ).map_err(|x| x + 4 * i)?;
    }

    // Reserve last four bytes to handle equals signs
    let remainder_group_start = n_remaining_groups - 1;

    let input_tail = &remainder_input[4 * remainder_group_start..];
    let output_tail = &mut remainder_output[3 * remainder_group_start..];
    let a = decode_one(input_tail[0]).map_err(|_| 4 * remainder_group_start)?;
    let b = decode_one(input_tail[1]).map_err(|_| 4 * remainder_group_start + 1)?;
    output_tail[0] = (a << 2) | (b >> 4);

    // handle padding
    if input_tail[2] != b'=' {
        let c = decode_one(input_tail[2]).map_err(|_| 4 * remainder_group_start + 2)?;
        output_tail[1] = (b << 4) | (c >> 2);

        if input_tail[3] != b'=' {
            let d = decode_one(input_tail[3]).map_err(|_| 4 * remainder_group_start + 3)?;
            output_tail[2] = (c << 6) | d;
        }
    }
    Ok(())
}
