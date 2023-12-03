use std::sync::Arc;
use std::thread::JoinHandle;

use crate::receiver::Receiver;
use crate::socket::Socket;
use crate::transmitter::Transmitter;

pub struct Window {
    transmitter: Arc<Transmitter>,

    receiver: Arc<Receiver>,

}

impl Window {
    pub fn new(local_addr: &str, remote_addr: &str) -> std::io::Result<Self> {
        let socket = Socket::bind(local_addr)?;
        socket.connect(remote_addr)?;
        let arc_socket = Arc::new(socket);
        let transmitter = Arc::new(Transmitter::new(arc_socket.clone()));
        Ok(Self {
            transmitter: transmitter.clone(),
            receiver: Arc::new(Receiver::new(arc_socket, transmitter)),

        })
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