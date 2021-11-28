pub trait Base64Encoder {
    fn encode(&self, buffer: &[u8], output: &mut [u8]);
}
