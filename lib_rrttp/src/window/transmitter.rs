use std::cmp::min;
use log::info;
use crate::constants::MAX_DATA_SIZE;
use crate::frame::Frame;
use crate::socket::Socket;
use crate::window::Window;

pub struct Transmitter {
    window: Window,
    pub socket: Socket,

    /// The highest sequence number that has been acknowledged.
    /// Also marks the beginning of the window.
    highest_acknowledged: u32,
}


impl Transmitter {
    pub fn new(local_addr: &str, remote_addr: &str) -> std::io::Result<Self> {
        let socket = Socket::bind(local_addr)?;
        socket.connect(remote_addr)?;
        Ok(Self {
            window: Window::default(),
            socket,
            highest_acknowledged: 0,
        })
    }

    pub fn send(&mut self, data_buffer: &[u8]) -> std::io::Result<usize> {
        let segments = data_buffer.len() as f32 / MAX_DATA_SIZE as f32;
        let segments = segments.ceil() as u32;

        let mut sequence_number = self.highest_acknowledged;

        for i in 0..self.window.size {
            sequence_number = self.highest_acknowledged + i;

            if sequence_number >= segments {
                break;
            }

            // Construct frame
            let mut frame = Frame::default();
            frame.set_sequence_number(sequence_number);


            frame.set_acknowledgment_number(0);
            frame.set_control_bits(0);
            // Set Data

            let buffer_shift = sequence_number * MAX_DATA_SIZE;

            let buffer_left = data_buffer.len() as u32 - buffer_shift;

            let data_lower_bound = buffer_shift as usize;
            let data_upper_bound = (buffer_shift + min(buffer_left, MAX_DATA_SIZE)) as usize;

            let data_segment = &data_buffer[data_lower_bound..data_upper_bound];
            frame.set_data(data_segment);


            // Send frame
            info!("Sent frame with sequence number {}", sequence_number);
            self.socket.send(frame.get_buffer())?;
        }
        Ok(0)
    }
}