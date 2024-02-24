use std::net::UdpSocket;
use std::sync::Arc;

use log::{error, info};

use crate::transport_layer::constants::MAX_FRAME_SIZE;
use crate::transport_layer::control_bits::ControlBits;
use crate::transport_layer::frame::{Frame, FrameType};

pub struct SocketAbstraction {
    socket: Arc<UdpSocket>,
    receive_thread: Option<std::thread::JoinHandle<()>>,
}

impl SocketAbstraction {
    pub fn bind(addr: &str) -> std::io::Result<SocketInterface> {
        let udp_socket = Arc::new(UdpSocket::bind(addr)?);
        let udp_socket_for_thread = udp_socket.clone();
        let (ack_sender, ack_receiver) = std::sync::mpsc::channel();
        let (data_sender, data_receiver) = std::sync::mpsc::channel();

        let thread = std::thread::spawn(move || loop {
            let mut buffer = [0; MAX_FRAME_SIZE];
            match udp_socket_for_thread.recv_from(&mut buffer) {
                Ok((size, _)) => {
                    let data = &buffer[..size];
                    let frame: Frame = data.into();
                    match frame.get_frame_type() {
                        FrameType::Data => {
                            let _ = data_sender.send(frame);
                        }
                        FrameType::Ack => {
                            let _ = ack_sender.send(frame.get_sequence_number());
                        }
                        _ => {
                            error!("Unknown frame type");
                        }
                    }
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }
        });
        let socket_abstraction = Self {
            socket: udp_socket,
            receive_thread: Some(thread),
        };

        Ok(SocketInterface {
            ack_receiver,
            data_receiver,
            socket: Arc::new(socket_abstraction),
        })
    }

    pub fn connect(&self, addr: &str) -> std::io::Result<()> {
        self.socket.connect(addr)
    }
    pub fn receive(&self) -> std::io::Result<(usize, [u8; MAX_FRAME_SIZE], std::net::SocketAddr)> {
        let mut buffer = [0; MAX_FRAME_SIZE];
        match self.socket.recv_from(&mut buffer) {
            Ok(result) => Ok((result.0, buffer, result.1)),
            Err(e) => Err(e),
        }
    }
    pub fn send(&self, buffer: &[u8]) -> std::io::Result<usize> {
        self.socket.send(buffer)
    }
    pub fn send_ack(&self, sequence_number: u32) -> std::io::Result<usize> {
        let mut frame = Frame::default();
        frame.set_sequence_number(sequence_number);
        frame.set_control_bits(ControlBits::ACK.bits());
        self.socket.send(frame.get_buffer())
    }
}

impl Drop for SocketAbstraction {
    fn drop(&mut self) {
        self.receive_thread.take().unwrap().join().unwrap();
    }
}

pub struct SocketInterface {
    pub ack_receiver: std::sync::mpsc::Receiver<u32>,
    pub data_receiver: std::sync::mpsc::Receiver<Frame>,
    pub socket: Arc<SocketAbstraction>,
}
