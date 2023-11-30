use crate::constants::{MAX_FRAME_SIZE, MIN_FRAME_SIZE};
use crate::option::{FrameOption, OptionKind};

#[derive(Debug)]
pub struct Frame {
    frame: [u8; MAX_FRAME_SIZE],
    data_size: usize,

    /// In bytes
    options_size: usize,
}

const SEQUENCE_NUMBER_OCTET: usize = 0;
const ACKNOWLEDGMENT_NUMBER_OCTET: usize = 4;
const CONTROL_BITS_OCTET: usize = 8;
const OPTIONS_OCTET: usize = 12;

impl Frame {
    pub fn set_sequence_number(&mut self, sequence_number: u32) {
        let net_sequence_number = sequence_number.to_be_bytes();
        self.frame[SEQUENCE_NUMBER_OCTET] = net_sequence_number[0];
        self.frame[SEQUENCE_NUMBER_OCTET + 1] = net_sequence_number[1];
        self.frame[SEQUENCE_NUMBER_OCTET + 2] = net_sequence_number[2];
        self.frame[SEQUENCE_NUMBER_OCTET + 3] = net_sequence_number[3];
    }

    pub fn get_sequence_number(&self) -> u32 {
        u32::from_be_bytes(self.frame[SEQUENCE_NUMBER_OCTET..SEQUENCE_NUMBER_OCTET + 4].try_into().expect("Failed to convert sequence number to u32"))
    }

    pub fn set_acknowledgment_number(&mut self, acknowledgment_number: u32) {
        let net_acknowledgment_number = acknowledgment_number.to_be_bytes();
        self.frame[ACKNOWLEDGMENT_NUMBER_OCTET] = net_acknowledgment_number[0];
        self.frame[ACKNOWLEDGMENT_NUMBER_OCTET + 1] = net_acknowledgment_number[1];
        self.frame[ACKNOWLEDGMENT_NUMBER_OCTET + 2] = net_acknowledgment_number[2];
        self.frame[ACKNOWLEDGMENT_NUMBER_OCTET + 3] = net_acknowledgment_number[3];
    }

    pub fn get_acknowledgment_number(&self) -> u32 {
        u32::from_be_bytes(self.frame[ACKNOWLEDGMENT_NUMBER_OCTET..ACKNOWLEDGMENT_NUMBER_OCTET + 4].try_into().expect("Failed to convert acknowledgment number to u32"))
    }

    pub fn set_control_bits(&mut self, control_bits: u8) {
        self.frame[CONTROL_BITS_OCTET] = control_bits;
    }

    pub fn get_control_bits(&self) -> u8 {
        self.frame[CONTROL_BITS_OCTET]
    }

    pub fn set_options(&mut self, options: &[FrameOption]) {
        self.options_size = 0;

        for option in options {
            let start_offset = OPTIONS_OCTET + self.options_size;
            self.frame[start_offset] = option.kind.clone() as u8;
            let len = option.data.len();
            self.frame[start_offset + 1] = len as u8;
            let data_offset = start_offset + 2;
            self.frame[data_offset..data_offset +len].copy_from_slice(option.data);
            self.options_size += 2 + len;
        }
    }

    pub fn get_options(&self) -> Option<Vec<FrameOption>> {
        let mut options = Vec::new();
        let offset = OPTIONS_OCTET;

        let options_size = self.get_data_offset() as usize - MIN_FRAME_SIZE;

        if options_size == 0 {
            return None;
        }
        let mut options_read = 0;
        while options_read < options_size {
            let kind:OptionKind = OptionKind::from(self.frame[offset]);
            let len = self.frame[offset + 1] as usize;
            let data_offset = offset + 2;
            let data = &self.frame[data_offset..data_offset + len];
            options.push(FrameOption::new(kind, data));
            options_read += 2 + len;
        }
        Some(options)
    }

    pub fn set_data_offset(&mut self, data_offset: u8) {
        self.frame[CONTROL_BITS_OCTET + 3] = data_offset;
    }

    pub fn get_data_offset(&self) -> u8 {
        self.frame[CONTROL_BITS_OCTET + 3]
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data_size = data.len();
        let offset = MIN_FRAME_SIZE + self.options_size;
        self.set_data_offset(offset as u8);
        self.frame[offset..(offset + self.data_size)].copy_from_slice(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.frame[self.get_data_offset() as usize..]
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.frame[0..MIN_FRAME_SIZE + self.options_size + self.data_size]
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            frame: [0; MAX_FRAME_SIZE],
            data_size: 0,
            options_size: 0,
        }
    }
}

impl From<&[u8]> for Frame {
    fn from(buffer: &[u8]) -> Self {
        let mut frame = Frame::default();
        frame.frame[..buffer.len()].copy_from_slice(buffer);
        frame
    }
}

impl From<[u8; MAX_FRAME_SIZE]> for Frame {
    fn from(value: [u8; MAX_FRAME_SIZE]) -> Self {
        Self {
            frame: value,
            data_size: 0,
            options_size: 0,
        }
    }
}