use crate::constants::MAX_FRAME_SIZE;

#[derive(Debug)]
pub struct Frame([u8; MAX_FRAME_SIZE / 8]);

const SEQUENCE_NUMBER_OCTET: usize = 0;
const ACKNOWLEDGMENT_NUMBER_OCTET: usize = 4;
const CONTROL_BITS_OCTET: usize = 8;

impl Frame {
    pub fn set_sequence_number(&mut self, sequence_number: u32) {
        let net_sequence_number = sequence_number.to_be_bytes();
        self.0[SEQUENCE_NUMBER_OCTET] = net_sequence_number[0];
        self.0[SEQUENCE_NUMBER_OCTET + 1] = net_sequence_number[1];
        self.0[SEQUENCE_NUMBER_OCTET + 2] = net_sequence_number[2];
        self.0[SEQUENCE_NUMBER_OCTET + 3] = net_sequence_number[3];
    }

    pub fn set_acknowledgment_number(&mut self, acknowledgment_number: u32) {
        let net_acknowledgment_number = acknowledgment_number.to_be_bytes();
        self.0[ACKNOWLEDGMENT_NUMBER_OCTET] = net_acknowledgment_number[0];
        self.0[ACKNOWLEDGMENT_NUMBER_OCTET + 1] = net_acknowledgment_number[1];
        self.0[ACKNOWLEDGMENT_NUMBER_OCTET + 2] = net_acknowledgment_number[2];
        self.0[ACKNOWLEDGMENT_NUMBER_OCTET + 3] = net_acknowledgment_number[3];
    }

    pub fn set_control_bits(&mut self, control_bits: u8) {
        self.0[CONTROL_BITS_OCTET] = control_bits;
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.0[16..(16+data.len())].copy_from_slice(data);
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.0
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self([0; MAX_FRAME_SIZE / 8])
    }
}