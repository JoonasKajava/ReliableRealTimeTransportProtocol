use crate::transport_layer::constants::{MAX_FRAME_SIZE, MIN_FRAME_SIZE};
use crate::transport_layer::control_bits::ControlBits;
use crate::transport_layer::option::{FrameOption, OptionKind};

#[derive(Debug, Clone)]
pub struct Frame {
    frame: [u8; MAX_FRAME_SIZE],
    data_length: usize,

    /// In bytes
    options_size: usize,
}

const SEQUENCE_NUMBER_OCTET: usize = 0;

const CONTROL_BITS_OCTET: usize = 4;

const DATA_OFFSET_OCTET: usize = 4;
const DATA_OFFSET_OFFSET: usize = 3;

const DATA_LENGTH_OCTET: usize = 4;
const DATA_LENGTH_OFFSET: usize = 2;

const OPTIONS_OCTET: usize = 8;

impl Frame {
    pub fn set_sequence_number(&mut self, sequence_number: u32) {
        let net_sequence_number = sequence_number.to_be_bytes();
        self.frame[SEQUENCE_NUMBER_OCTET..4].copy_from_slice(&net_sequence_number);
    }

    pub fn get_sequence_number(&self) -> u32 {
        u32::from_be_bytes(
            self.frame[SEQUENCE_NUMBER_OCTET..SEQUENCE_NUMBER_OCTET + 4]
                .try_into()
                .expect("Failed to convert sequence number to u32"),
        )
    }

    pub fn set_control_bits(&mut self, control_bits: u8) {
        self.frame[CONTROL_BITS_OCTET] = control_bits;
    }

    pub fn get_control_bits(&self) -> u8 {
        self.frame[CONTROL_BITS_OCTET]
    }

    pub fn get_frame_type(&self) -> FrameType {
        let control_bits = ControlBits::from_bits(self.get_control_bits());
        match control_bits {
            None => FrameType::Unknown,
            Some(bits) => {
                if bits.contains(ControlBits::ACK) {
                    FrameType::Ack
                } else {
                    FrameType::Data
                }
            }
        }
    }

    pub fn set_options(&mut self, options: &[FrameOption]) {
        self.options_size = 0;

        for option in options {
            let start_offset = OPTIONS_OCTET + self.options_size;
            self.frame[start_offset] = option.kind.clone() as u8;
            let len = option.data.len();
            self.frame[start_offset + 1] = len as u8;
            let data_offset = start_offset + 2;
            self.frame[data_offset..data_offset + len].copy_from_slice(option.data);
            self.options_size += 2 + len;
        }
    }

    /// call before setting data
    pub fn append_option(&mut self, option: FrameOption) {
        let start_offset = OPTIONS_OCTET + self.options_size;
        self.frame[start_offset] = option.kind.clone() as u8;
        let len = option.data.len();
        self.frame[start_offset + 1] = len as u8;
        let data_offset = start_offset + 2;
        self.frame[data_offset..data_offset + len].copy_from_slice(option.data);
        self.options_size += 2 + len;
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
            let kind: OptionKind = OptionKind::from(self.frame[offset]);
            let len = self.frame[offset + 1] as usize;
            let data_offset = offset + 2;
            let data = &self.frame[data_offset..data_offset + len];
            options.push(FrameOption::new(kind, data));
            options_read += 2 + len;
        }
        Some(options)
    }

    pub fn set_data_length(&mut self, data_length: u8) {
        self.frame[DATA_LENGTH_OCTET + DATA_LENGTH_OFFSET] = data_length;
    }

    pub fn get_data_length(&self) -> u8 {
        self.frame[DATA_LENGTH_OCTET + DATA_LENGTH_OFFSET]
    }

    pub fn set_data_offset(&mut self, data_offset: u8) {
        self.frame[DATA_OFFSET_OCTET + DATA_OFFSET_OFFSET] = data_offset;
    }

    pub fn get_data_offset(&self) -> u8 {
        self.frame[DATA_OFFSET_OCTET + DATA_OFFSET_OFFSET]
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data_length = data.len();
        let offset = MIN_FRAME_SIZE + self.options_size;
        self.set_data_offset(offset as u8);

        self.set_data_length(self.data_length as u8);
        self.frame[offset..(offset + self.data_length)].copy_from_slice(data);
    }
    pub fn get_data(&self) -> &[u8] {
        let offset = self.get_data_offset() as usize;
        let length = self.get_data_length() as usize;
        &self.frame[offset..offset + length]
    }

    /// Returns the buffer.
    /// The buffer is a slice of the frame.
    pub fn get_buffer(&self) -> &[u8] {
        &self.frame[0..MIN_FRAME_SIZE + self.options_size + self.data_length]
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            frame: [0; MAX_FRAME_SIZE],
            data_length: 0,
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
            data_length: 0,
            options_size: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FrameType {
    Data,
    Ack,
    Unknown,
}
