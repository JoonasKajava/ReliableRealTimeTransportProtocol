use crate::constants::WINDOW_SIZE;

pub mod transmitter;
mod receiver;

pub struct Window {
    sequence_number: usize,
    size: usize,
}


impl Default for Window {
    fn default() -> Self {
        Self {
            sequence_number: 0,
            size: WINDOW_SIZE,
        }
    }
}