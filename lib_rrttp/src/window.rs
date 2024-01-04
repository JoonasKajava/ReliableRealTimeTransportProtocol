use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

use crate::receiver::Receiver;
use crate::socket::Socket;
use crate::transmitter::Transmitter;

pub struct Window {
    transmitter: Arc<Transmitter>,

    receiver: Receiver,

}

impl Default for Window {
    fn default() -> Self {
        let socket = Socket::new();
        let arc_socket = Arc::new(RwLock::new(socket));
        let transmitter = Arc::new(Transmitter::new(arc_socket.clone()));
        let receiver = Receiver::new(arc_socket, transmitter.clone());
        Self {
            transmitter,
            receiver,

        }
    }
}

impl Window {
    pub fn bind_local_socket(&mut self, local_addr: &str) -> std::io::Result<()> {
        self.receiver.bind(local_addr)
    }

    pub fn connect(&self, remote_addr: &str) -> std::io::Result<()> {
        self.transmitter.connect(remote_addr)
    }

    pub fn listen(&self) -> JoinHandle<()> {
        self.receiver.listen()
    }

    pub fn incoming_messages(&self) -> &std::sync::mpsc::Receiver<Vec<u8>> {
        self.receiver.incoming_messages()
    }


    pub fn send(&mut self, data_buffer: &[u8]) -> std::io::Result<usize> {
        self.transmitter.send(data_buffer)
    }
}