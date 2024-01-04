use std::net::UdpSocket;

use crate::constants::MAX_FRAME_SIZE;

pub struct Socket {
    socket: Option<UdpSocket>
}


impl Socket {
    pub fn new() -> Self {
        Self {
            socket: None
        }
    }

    pub fn bind(&mut self, addr: &str) -> std::io::Result<()> {
        let udp_socket = UdpSocket::bind(addr)?;
        self.socket = Some(udp_socket);
        Ok(())
    }

    pub fn connect(&self, addr: &str) -> std::io::Result<()>
    {
        match &self.socket {
            None => Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "Socket not bound")),
            Some(socket) => socket.connect(addr)
        }
    }
    pub fn receive(&self) -> std::io::Result<(usize, [u8; MAX_FRAME_SIZE ], std::net::SocketAddr)> {
        let socket = match &self.socket {
            None => return Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "Socket not bound")),
            Some(socket) => socket
        };

        let mut buffer = [0; MAX_FRAME_SIZE];
        match socket.recv_from(&mut buffer) {
            Ok(result) => {
                Ok((result.0, buffer, result.1))
            }
            Err(e) => Err(e)
        }
    }

    pub fn send(&self, buffer: &[u8]) -> std::io::Result<usize> {
        match &self.socket {
            None => Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "Socket not bound")),
            Some(socket) => socket.send(buffer)
        }
    }
}