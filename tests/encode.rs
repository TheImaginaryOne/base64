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
        }
    }
}

gen_tests!(basic_encoder_test, base64::basic::BasicEncoder);
//gen_tests!(fast_encoder_test, base64::fast::FastEncoder);
