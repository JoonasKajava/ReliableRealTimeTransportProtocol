use crate::constants::WINDOW_SIZE;

pub mod transmitter;
pub mod receiver;

pub struct Window {
    sequence_number: usize,
    size: u32,
}


impl Default for Window {
    fn default() -> Self {
        Self {
            sequence_number: 0,
            size: WINDOW_SIZE,
        }
    }
}