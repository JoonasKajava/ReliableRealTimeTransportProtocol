use crate::constants::{MAX_FRAME_SIZE, MIN_FRAME_SIZE};

#[derive(Debug)]
pub struct Frame {
    frame: [u8; MAX_FRAME_SIZE as usize],
    data_size: usize,
    options_size: usize,
}

const SEQUENCE_NUMBER_OCTET: usize = 0;
const ACKNOWLEDGMENT_NUMBER_OCTET: usize = 4;
const CONTROL_BITS_OCTET: usize = 8;

impl Frame {
    pub fn set_sequence_number(&mut self, sequence_number: u32) {
        let net_sequence_number = sequence_number.to_be_bytes();
        self.frame[SEQUENCE_NUMBER_OCTET] = net_sequence_number[0];
        self.frame[SEQUENCE_NUMBER_OCTET + 1] = net_sequence_number[1];
        self.frame[SEQUENCE_NUMBER_OCTET + 2] = net_sequence_number[2];
        self.frame[SEQUENCE_NUMBER_OCTET + 3] = net_sequence_number[3];
    }

    pub fn get_sequence_number(&self) -> u32 {
        u32::from_be_bytes(self.frame[SEQUENCE_NUMBER_OCTET..SEQUENCE_NUMBER_OCTET+4].try_into().expect("Failed to convert sequence number to u32"))
    }

    pub fn set_acknowledgment_number(&mut self, acknowledgment_number: u32) {
        let net_acknowledgment_number = acknowledgment_number.to_be_bytes();
        self.frame[ACKNOWLEDGMENT_NUMBER_OCTET] = net_acknowledgment_number[0];
        self.frame[ACKNOWLEDGMENT_NUMBER_OCTET + 1] = net_acknowledgment_number[1];
        self.frame[ACKNOWLEDGMENT_NUMBER_OCTET + 2] = net_acknowledgment_number[2];
        self.frame[ACKNOWLEDGMENT_NUMBER_OCTET + 3] = net_acknowledgment_number[3];
    }

    pub fn get_acknowledgment_number(&self) -> u32 {
        u32::from_be_bytes(self.frame[ACKNOWLEDGMENT_NUMBER_OCTET..ACKNOWLEDGMENT_NUMBER_OCTET+4].try_into().expect("Failed to convert acknowledgment number to u32"))
    }

    pub fn set_control_bits(&mut self, control_bits: u8) {
        self.frame[CONTROL_BITS_OCTET] = control_bits;
    }

    pub fn get_control_bits(&self) -> u8 {
        self.frame[CONTROL_BITS_OCTET]
    }

    pub fn set_data_offset(&mut self, data_offset: u8) {
        self.frame[CONTROL_BITS_OCTET + 3] = data_offset;
    }

    pub fn get_data_offset(&self) -> u8 {
        self.frame[CONTROL_BITS_OCTET + 3]
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data_size = data.len();
        let offset = MIN_FRAME_SIZE as usize + self.options_size;
        self.set_data_offset(offset as u8);
        self.frame[offset..(offset + self.data_size)].copy_from_slice(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.frame[self.get_data_offset() as usize..]
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.frame[0.. MIN_FRAME_SIZE as usize + self.options_size + self.data_size]
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            frame: [0; MAX_FRAME_SIZE as usize],
            data_size: 0,
            options_size: 0,
        }
    }
}