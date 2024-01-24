use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::transport_layer::control_bits::ControlBits;
use crate::transport_layer::frame::Frame;
use crate::transport_layer::receiving_window_manager::ReceivingWindowManager;
use crate::transport_layer::socket::Socket;


pub enum PacketType {
    Complete(Frame),
    // These fragments are always in order
    Fragment(Frame),
    Ack,
}

pub struct ConnectionManager {
    message_queue: Sender<Vec<u8>>,
}

impl ConnectionManager {
    pub fn start(local_addr: &str) -> std::io::Result<(Self, Receiver<PacketType>)> {
        let (message_queue_sender, messages_to_send) = std::sync::mpsc::channel();

        let (incoming_message_sender, incoming_message_receiver) = std::sync::mpsc::channel();

        let listen_socket = Arc::new(Socket::bind(local_addr)?);
        let ack_socket = listen_socket.clone();
        let sender_socket = listen_socket.clone();
        thread::spawn(move || {
            let mut receiving_window_manager = ReceivingWindowManager::default();
            loop {
                receiving_window_manager.shift_all_windows().into_iter().for_each(|(channel_id, fragments_or_message)| {
                    // TODO: Handle incoming messages where they are in order, but k != 0 are not complete
                    for i in fragments_or_message.into_iter() {
                        let packet = match channel_id {
                            0 => PacketType::Complete(i),
                            _ => PacketType::Fragment(i),
                        };
                        incoming_message_sender.send(packet).unwrap();
                    }
                });

                let (size, buffer, addr) = listen_socket.receive().unwrap();

                let frame: Frame = buffer.into();

                let control_bits = ControlBits::from_bits(frame.get_control_bits()).expect("Failed to parse control bits");

                let channel_id = frame.get_channel_id();

                let sequence_number = frame.get_sequence_number();

                if !control_bits.contains(ControlBits::ACK) {
                    let ack_frame = ConnectionManager::construct_ack_frame(channel_id, sequence_number);
                    ack_socket.send(ack_frame.get_buffer()).unwrap();

                    receiving_window_manager.handle_incoming_frame(frame);
                } else {
                    incoming_message_sender.send(PacketType::Ack).unwrap();
                }


                println!("Received {} bytes from {}", size, addr);
            }
        });

        thread::spawn(move || {
            loop {
                let next_message: Vec<u8> = messages_to_send.recv().unwrap();
                sender_socket.send(next_message.as_slice()).unwrap();
            }
        });
        Ok((Self {
            message_queue: message_queue_sender,
        }, incoming_message_receiver))
    }

    fn construct_ack_frame(channel_id: u8, sequence_number: u32) -> Frame {
        let mut frame = Frame::default();
        frame.set_sequence_number(sequence_number);
        frame.set_control_bits(ControlBits::ACK.bits());
        frame.set_channel_id(channel_id);
        frame
    }
}