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
    pub fn new(addr: &str) -> std::io::Result<Self> {
        Ok(Self {
            window: Window::default(),
            socket: Socket::bind(addr)?,
            highest_acknowledged: 0,
        })
    }

    pub fn send(&mut self, data: &[u8]) -> std::io::Result<usize> {
        let segments = data.len() as f32 / (MAX_DATA_SIZE as f32 / 8f32);
        let segments = segments.ceil() as usize;
        let mut current_segment = 0;
        while current_segment < segments {
            let mut frame = Frame::default();
            frame.set_sequence_number(self.highest_acknowledged + 1);

            let data_lower_bound = current_segment * (MAX_DATA_SIZE / 8);
            let data_upper_bound = min(data.len(), (current_segment + 1) * (MAX_DATA_SIZE / 8));


            let data_segment = &data[data_lower_bound..data_upper_bound];
            frame.set_data(data_segment);
            info!("Sent frame with sequence number {}", self.highest_acknowledged + 1);
            self.socket.send(frame.get_buffer())?;
            current_segment += 1;
        }
        Ok(0)
    }
}