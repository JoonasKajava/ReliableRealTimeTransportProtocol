use std::fs;
use std::sync::Arc;
use std::thread::JoinHandle;
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