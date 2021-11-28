use crate::traits::Base64Encoder;

pub struct FastEncoder {}

impl FastEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Base64Encoder for FastEncoder {
    fn encode(&self, _buffer: &[u8], _output: &mut [u8]) {
        unimplemented!()
    }
}
