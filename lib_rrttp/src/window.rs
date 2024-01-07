use std::sync::Arc;
use std::thread::JoinHandle;

use crate::receiver::Receiver;
use crate::socket::Socket;
use crate::transmitter::Transmitter;

pub struct Window {
    transmitter: Arc<Transmitter>,

    socket: Arc<Socket>,

    listen_handle: JoinHandle<()>,

    receiver: Receiver,

}

impl Window {
    pub fn new(local_addr: &str) -> std::io::Result<(Window, std::sync::mpsc::Receiver<Vec<u8>>)> {
        let socket = Arc::new(Socket::bind(local_addr)?);
        let transmitter = Arc::new(Transmitter::new(socket.clone()));
        let receiver = Receiver::new(socket.clone(), transmitter.clone());
        Ok((Self {
            socket,
            transmitter,
            listen_handle: receiver.0.listen(),
            receiver: receiver.0,
        }, receiver.1))
    }
}

impl Window {
    pub fn connect(&self, remote_addr: &str) -> std::io::Result<()> {
        self.transmitter.connect(remote_addr)
    }

    pub fn send(&mut self, data_buffer: &[u8]) -> std::io::Result<usize> {
        self.transmitter.send(data_buffer)
    }
}