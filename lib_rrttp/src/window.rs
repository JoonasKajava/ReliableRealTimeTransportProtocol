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
        let transmitter1 = Arc::new(Transmitter::new(socket.clone()));
        let receiver1 = Receiver::new(socket.clone(), transmitter1.clone());
        Ok((Self {
            socket,
            transmitter: transmitter1,
            listen_handle: receiver1.0.listen(),
            receiver: receiver1.0,
        }, receiver1.1))
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