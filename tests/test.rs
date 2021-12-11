use base64::traits::*;

fn test_decode_internal<T: Base64Decoder>(decoder: &T, input: &[u8], expected_output: &[u8]) {
    let mut output_len = (input.len() / 4) * 3;
    if input.len() >= 1 && input[input.len() - 1] == b'=' {
        output_len -= 1;
        if input.len() >= 2 && input[input.len() - 2] == b'=' {
            output_len -= 1;
        }
    }
    let mut output = vec![0; output_len];

    decoder.decode(&input, &mut output).unwrap();
    assert_eq!(expected_output, output);
}

fn test_encode_internal<T: Base64Encoder>(encoder: &T, input: &[u8], expected_output: &[u8]) {
    let mut output = vec![0; 4 * ((input.len() + 2) / 3)];

    encoder.encode(&input, &mut output);
    assert_eq!(expected_output, output);
}

macro_rules! test_cases {
    // wrap the function in parameterized tests.
    ($f:item) => {
        #[parameterized(test_case = {
            (b"", b""),
            (b"a", b"YQ=="),
            (b"ouch, my foot hurts! Someone, please help!!", b"b3VjaCwgbXkgZm9vdCBodXJ0cyEgU29tZW9uZSwgcGxlYXNlIGhlbHAhIQ=="),
            (b"ouch, my foot hurts! Someone, please help!", b"b3VjaCwgbXkgZm9vdCBodXJ0cyEgU29tZW9uZSwgcGxlYXNlIGhlbHAh"),
            (b"ouch, my foot hurts! Someone, please help", b"b3VjaCwgbXkgZm9vdCBodXJ0cyEgU29tZW9uZSwgcGxlYXNlIGhlbHA="),
            (b"ouch, my foot hurts! Someone, please help", b"b3VjaCwgbXkgZm9vdCBodXJ0cyEgU29tZW9uZSwgcGxlYXNlIGhlbHA="),
            // exactly 12 * 8 * 2 bytes
            (b"3bc3d1453b52dd2eaeeca3a5b125cb11de3dad443ca43e4bc51243c135ba23c4c142acd12e21e5e3d4d11ea1dea3cdec4a421d4bac1a4233b31de3223e13134412ed42e3d42d4aea5cebd1343b4c555eb3ee2cb4ea2a3b4e2cd5a5c4cc3b215d", b"M2JjM2QxNDUzYjUyZGQyZWFlZWNhM2E1YjEyNWNiMTFkZTNkYWQ0NDNjYTQzZTRiYzUxMjQzYzEzNWJhMjNjNGMxNDJhY2QxMmUyMWU1ZTNkNGQxMWVhMWRlYTNjZGVjNGE0MjFkNGJhYzFhNDIzM2IzMWRlMzIyM2UxMzEzNDQxMmVkNDJlM2Q0MmQ0YWVhNWNlYmQxMzQzYjRjNTU1ZWIzZWUyY2I0ZWEyYTNiNGUyY2Q1YTVjNGNjM2IyMTVk"),
            (b"3bc3d1453b52dd2eaeeca3a5b125cb11de3dad443ca43e4bc51243c135ba23c4c142acd12e21e5e3d4d11ea1dea3cdec4a421d4bac1a4233b31de3223e13134412ed42e3d42d4aea5cebd1343b4c555eb3ee2cb4ea2a3b4e2cd5a5c4cc3b215dz", b"M2JjM2QxNDUzYjUyZGQyZWFlZWNhM2E1YjEyNWNiMTFkZTNkYWQ0NDNjYTQzZTRiYzUxMjQzYzEzNWJhMjNjNGMxNDJhY2QxMmUyMWU1ZTNkNGQxMWVhMWRlYTNjZGVjNGE0MjFkNGJhYzFhNDIzM2IzMWRlMzIyM2UxMzEzNDQxMmVkNDJlM2Q0MmQ0YWVhNWNlYmQxMzQzYjRjNTU1ZWIzZWUyY2I0ZWEyYTNiNGUyY2Q1YTVjNGNjM2IyMTVkeg=="),
            (b"To be, or not to be, that is the question:
Whether 'tis nobler in the mind to suffer
The slings and arrows of outrageous fortune,
Or to take Arms against a Sea of troubles,
And by opposing end them: to die, to sleep
No more; and by a sleep, to say we end
The heart-ache, and the thousand natural shocks
That Flesh is heir to? 'Tis a consummation
Devoutly to be wished.", b"VG8gYmUsIG9yIG5vdCB0byBiZSwgdGhhdCBpcyB0aGUgcXVlc3Rpb246CldoZXRoZXIgJ3RpcyBub2JsZXIgaW4gdGhlIG1pbmQgdG8gc3VmZmVyClRoZSBzbGluZ3MgYW5kIGFycm93cyBvZiBvdXRyYWdlb3VzIGZvcnR1bmUsCk9yIHRvIHRha2UgQXJtcyBhZ2FpbnN0IGEgU2VhIG9mIHRyb3VibGVzLApBbmQgYnkgb3Bwb3NpbmcgZW5kIHRoZW06IHRvIGRpZSwgdG8gc2xlZXAKTm8gbW9yZTsgYW5kIGJ5IGEgc2xlZXAsIHRvIHNheSB3ZSBlbmQKVGhlIGhlYXJ0LWFjaGUsIGFuZCB0aGUgdGhvdXNhbmQgbmF0dXJhbCBzaG9ja3MKVGhhdCBGbGVzaCBpcyBoZWlyIHRvPyAnVGlzIGEgY29uc3VtbWF0aW9uCkRldm91dGx5IHRvIGJlIHdpc2hlZC4=")
        })]
        $f
    }
}

macro_rules! gen_tests {
    (encode, $test_name:ident, $A:ty) => {
        #[cfg(test)]
        mod $test_name {
            use super::*;
            use parameterized::parameterized;

            test_cases!(
                fn test_encode(test_case: (&[u8], &[u8])) {
                    test_encode_internal(&<$A>::new(), test_case.0, test_case.1);
                }
            );
        }
    };
    (decode, $test_name:ident, $A:ty) => {
        #[cfg(test)]
        mod $test_name {
            use super::*;
            use parameterized::parameterized;

            test_cases!(
                fn test_decode(test_case: (&[u8], &[u8])) {
                    test_decode_internal(&<$A>::new(), test_case.1, test_case.0);
                }
            );
        }
    };
}

gen_tests!(encode, basic_encoder_test, base64::basic::BasicEncoder);
gen_tests!(encode, fast_encoder_test, base64::fast::FastEncoder);

gen_tests!(decode, basic_decoder_test, base64::basic::BasicDecoder);
//gen_tests!(decode, fast_decoder_test, base64::fast::FastDecoder);
