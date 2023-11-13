
pub struct Frame([u8]);

impl Frame {
    pub fn set_sequence_number(&mut self, sequence_number: u32) {
        let net_sequence_number = sequence_number.to_be_bytes();
        let (left, _) = self.0.split_at_mut(4);
        left.copy_from_slice(&net_sequence_number);
    }
}