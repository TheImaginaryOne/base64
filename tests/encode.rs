use base64::traits::Base64Encoder;

fn test_encode<T: Base64Encoder>(encoder: &T, input: &[u8], expected_output: &[u8]) {
    let mut output = vec![0; 4 * ((input.len() + 2) / 3)];

    encoder.encode(&input, &mut output);
    assert_eq!(expected_output, output);
}

macro_rules! gen_tests {
    ($test_name:ident, $A:ty) => {
        #[cfg(test)]
        mod $test_name {
            use super::test_encode;
            #[test]
            fn empty() {
                test_encode(& <$A>::new(), b"", b"");
            }
            #[test]
            fn single() {
                test_encode(& <$A>::new(), b"a", b"YQ==");
            }
            #[test]
            fn basic() {
                test_encode(& <$A>::new(), b"light w", b"bGlnaHQgdw==");
            }
            #[test]
            fn basic_2() {
                test_encode(& <$A>::new(), b"light wo", b"bGlnaHQgd28=");
            }
            #[test]
            fn basic_3() {
                test_encode(& <$A>::new(), b"light wor", b"bGlnaHQgd29y");
            }
            // Subsequent tests test the fast implementation which converts 16 bytes at a time.
            #[test]
            fn ouch() {
                test_encode(& <$A>::new(), b"ouch my foot hurts!", b"b3VjaCBteSBmb290IGh1cnRzIQ==");
            }
            #[test]
            fn ouchhhh() {
                test_encode(& <$A>::new(), b"ouch, my foot hurts! Someone, please help!!", b"b3VjaCwgbXkgZm9vdCBodXJ0cyEgU29tZW9uZSwgcGxlYXNlIGhlbHAhIQ==");
            }
            #[test]
            fn shakespeare() {
                test_encode(& <$A>::new(), b"To be, or not to be, that is the question:
Whether 'tis nobler in the mind to suffer
The slings and arrows of outrageous fortune,
Or to take Arms against a Sea of troubles,
And by opposing end them: to die, to sleep
No more; and by a sleep, to say we end
The heart-ache, and the thousand natural shocks
That Flesh is heir to? 'Tis a consummation
Devoutly to be wished.", b"VG8gYmUsIG9yIG5vdCB0byBiZSwgdGhhdCBpcyB0aGUgcXVlc3Rpb246CldoZXRoZXIgJ3RpcyBub2JsZXIgaW4gdGhlIG1pbmQgdG8gc3VmZmVyClRoZSBzbGluZ3MgYW5kIGFycm93cyBvZiBvdXRyYWdlb3VzIGZvcnR1bmUsCk9yIHRvIHRha2UgQXJtcyBhZ2FpbnN0IGEgU2VhIG9mIHRyb3VibGVzLApBbmQgYnkgb3Bwb3NpbmcgZW5kIHRoZW06IHRvIGRpZSwgdG8gc2xlZXAKTm8gbW9yZTsgYW5kIGJ5IGEgc2xlZXAsIHRvIHNheSB3ZSBlbmQKVGhlIGhlYXJ0LWFjaGUsIGFuZCB0aGUgdGhvdXNhbmQgbmF0dXJhbCBzaG9ja3MKVGhhdCBGbGVzaCBpcyBoZWlyIHRvPyAnVGlzIGEgY29uc3VtbWF0aW9uCkRldm91dGx5IHRvIGJlIHdpc2hlZC4=");
            }
        }
    }
}

gen_tests!(basic_encoder_test, base64::basic::BasicEncoder);
gen_tests!(fast_encoder_test, base64::fast::FastEncoder);
