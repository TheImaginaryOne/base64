#[derive(Debug, PartialEq, Eq)]
pub enum DecoderError {
    InvalidLength,
    InvalidByte
}

pub trait Base64Encoder {
    fn encode(&self, buffer: &[u8], output: &mut [u8]);
}

pub trait Base64Decoder {
    fn decode(&self, buffer: &[u8], output: &mut [u8]) -> Result<(), DecoderError>;
}

