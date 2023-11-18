use std::net::UdpSocket;
use crate::constants::BUFFER_SIZE;

pub struct Socket {
    socket: UdpSocket,
    buffer: [u8; BUFFER_SIZE as usize],
}


impl Socket {
    pub fn bind(addr: &str) -> std::io::Result<Self> {
        let udp_socket = UdpSocket::bind(addr)?;
        Ok(Self {
            socket: udp_socket,
            buffer: [0; BUFFER_SIZE as usize],
        })
    }

    pub fn connect(&self, addr: &str) -> std::io::Result<()> {
        self.socket.connect(addr)
    }
    pub fn receive(&mut self) -> std::io::Result<(usize, &[u8], std::net::SocketAddr)> {
        match self.socket.recv_from(&mut self.buffer) {
            Ok(result) => {
                let slice = &self.buffer[..result.0];
                Ok((result.0, slice, result.1))
            }
            Err(e) => Err(e)
        }
    }

    pub fn send(&self, buffer: &[u8]) -> std::io::Result<usize> {
        self.socket.send(buffer)
    }
}