use std::net::UdpSocket;

use crate::constants::MAX_FRAME_SIZE;

pub struct Socket {
    socket: UdpSocket,
}


impl Socket {
    pub fn bind(addr: &str) -> std::io::Result<Self> {
        let udp_socket = UdpSocket::bind(addr)?;
        Ok(Self {
            socket: udp_socket
        })
    }

    pub fn connect(&self, addr: &str) -> std::io::Result<()>
    {
        self.socket.connect(addr)

    }
    pub fn receive(&self) -> std::io::Result<(usize, [u8; MAX_FRAME_SIZE], std::net::SocketAddr)> {
        let mut buffer = [0; MAX_FRAME_SIZE];
        match self.socket.recv_from(&mut buffer) {
            Ok(result) => {
                Ok((result.0, buffer, result.1))
            }
            Err(e) => Err(e)
        }
    }

    pub fn send(&self, buffer: &[u8]) -> std::io::Result<usize> {
        self.socket.send(buffer)
    }
}