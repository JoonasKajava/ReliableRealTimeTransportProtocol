use std::fs;
use std::sync::Arc;
use std::thread::JoinHandle;
use crate::transport_layer::connection_manager::SequenceNumber;
use crate::transport_layer::constants::MAX_FRAME_SIZE;

use crate::transport_layer::frame::Frame;
use crate::transport_layer::receiver::Receiver;
use crate::transport_layer::socket::Socket;
use crate::transport_layer::transmitter::Transmitter;

pub struct Window {
    transmitter: Transmitter,

    socket: Socket,

    receiver: Receiver,

}

impl Window {
    pub fn new(local_addr: &str) -> std::io::Result<(Window, std::sync::mpsc::Receiver<Vec<u8>>)> {
        let socket = Socket::bind(local_addr)?;
        let transmitter = Transmitter::new();
        let receiver = Receiver::new();
        Ok((Self {
            socket,
            transmitter,
            receiver: receiver.0,
        }, receiver.1))
    }

    pub fn listen(window: Arc<Window>) -> JoinHandle<()> {
        window.receiver.listen(window.clone())
    }

    pub fn receive(&self) -> std::io::Result<(usize, [u8; MAX_FRAME_SIZE], std::net::SocketAddr)> {
        self.socket.receive()
    }

    pub fn connect(&self, addr: &str) -> std::io::Result<()> {
        self.socket.connect(addr)
    }

    pub fn send_frame(&self, frame: Frame) -> std::io::Result<usize> {
        self.socket.send(frame.get_buffer())
    }

    pub fn handle_acknowledgment(&self, acknowledgment_number: u32) {
        self.transmitter.handle_acknowledgment(acknowledgment_number);
    }

    pub fn send_ack(&self, sequence_number: u32) {
        self.transmitter.send_ack(sequence_number, self);
    }

    pub fn send(&self, data: &[u8]) -> std::io::Result<usize> {
        self.transmitter.send(data, self)
    }

    pub fn send_file(&self, file_path: &str) -> std::io::Result<usize> {
        let file = fs::read(file_path)?;
        self.transmitter.send(file.as_slice(), self)
    }
}


#[derive(Default)]
pub struct NewWindow {
    // TODO: Transmitter should have same kinda stuff, but for tracking time
    frame_status: Vec<bool>,
    window_size: u32,
    window_left_edge: u32,
}

// There should be generic window that handles the frame status and the shifting
// Then there should be a transmitter and receiver that use the window


impl NewWindow {

    pub fn set_window_size(&mut self, size: u32) {
        self.window_size = size;
        self.frame_status.resize(size as usize, false);
    }

    pub fn get_window_size(&self) -> u32 {
        self.window_size
    }

    pub fn get_window_left_edge(&self) -> u32 {
        self.window_left_edge
    }
    
    pub fn get_window_index(&self, sequence_number: SequenceNumber) -> usize {
        (sequence_number - self.window_left_edge) as usize
    }

    pub fn shift_window(&mut self) -> usize {
        let mut shift_amount = 0usize;
        for e in self.frame_status.iter() {
            if *e {
                shift_amount += 1;
            } else {
                break;
            }
        }
        self.frame_status.drain(0..shift_amount);
        self.window_left_edge += shift_amount as u32;
        shift_amount
        //TODO Here we should send the messages in order to the application WHEN IN CONTEXT OF MAIN CHANNEL
    }

    pub fn is_within_window(&self, sequence_number: u32) -> bool {
        sequence_number >= self.window_left_edge && sequence_number < self.window_left_edge + self.window_size
    }

    pub fn update_frame_status(&mut self, index: usize) {
        self.frame_status[index] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_shift_when_frame_status_empty() {
        let mut window = NewWindow {
            frame_status: vec![],
            window_left_edge: 0,
            ..Default::default()
        };

        window.shift_window();

        assert_eq!(window.window_left_edge, 0);
    }
    #[test]
    fn no_shift_when_frame_status_full_of_false() {
        let mut window = NewWindow {
            frame_status: vec![false, false, false, false, false],
            window_left_edge: 0,
            ..Default::default()
        };

        window.shift_window();

        assert_eq!(window.window_left_edge, 0);
    }

    #[test]
    fn shift_when_possible() {
        let mut window = NewWindow {
            frame_status: vec![true, true, false, true, false],
            window_left_edge: 0,
            ..Default::default()
        };

        window.shift_window();

        assert_eq!(window.window_left_edge, 2);
        assert_eq!(window.frame_status, vec![false, true, false]);

        window.shift_window();

        assert_eq!(window.window_left_edge, 2);
        assert_eq!(window.frame_status, vec![false, true, false]);

    }
}

