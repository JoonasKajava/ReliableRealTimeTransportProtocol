use std::str::from_utf8;
use log::info;
use crate::frame::Frame;
use crate::socket::Socket;
use crate::window::Window;

pub struct Receiver {
    window: Window,
    socket: Socket,
    highest_received: usize,
}

impl Receiver {
    pub fn new(local_addr: &str, remote_addr: &str) -> std::io::Result<Self> {
        let socket = Socket::bind(local_addr)?;
        socket.connect(remote_addr)?;
        Ok(Self {
            window: Window::default(),
            highest_received: 0,
            socket,
        })
    }
    pub fn read(&mut self) {
        loop {
            let (_, buffer, _) = self.socket.receive().expect("Failed to receive");
            let frame: Frame = buffer.into();
            let sequence_number = frame.get_sequence_number() as usize;
            let data = frame.get_data();

            info!("Received frame with sequence number {} data: {}", sequence_number, from_utf8(data).unwrap());
        }
    }
}