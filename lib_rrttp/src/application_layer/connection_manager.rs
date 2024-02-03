use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender};
use std::sync::Arc;
use std::thread;

use crate::transport_layer::control_bits::ControlBits;
use crate::transport_layer::frame::Frame;
use crate::transport_layer::receiver_window::ReceiverWindow;
use crate::transport_layer::socket::Socket;
use crate::transport_layer::transmitter_window::TransmitterWindow;

pub enum ConnectionEventType {
    ReceivedFrame(Frame),
    ReceivedCompleteMessage(Vec<u8>),

    SentMessage,
    SentFrame(Frame),

    ReceivedAck(Frame),
    SentAck(Frame),
}

pub type SequenceNumber = u32;

pub struct ConnectionManager {
    listener_thread: Option<thread::JoinHandle<()>>,
    transmitter_thread: Option<thread::JoinHandle<()>>,
}

impl ConnectionManager {
    pub fn start(
        local_addr: &str,
    ) -> std::io::Result<(Self, Receiver<ConnectionEventType>, SyncSender<Vec<u8>>)> {
        let (connection_events_sender, connection_events_receiver) = channel();

        let (messages_to_send, messages_to_send_receiver) = sync_channel(100);

        let (ack_sender, ack_receiver) = channel();

        let listen_socket = Arc::new(Socket::bind(local_addr)?);

        let ack_socket = listen_socket.clone();
        let sender_socket = listen_socket.clone();

        let connection_events_sender_receiver = connection_events_sender.clone();
        let connection_events_sender_transmitter = connection_events_sender.clone();

        let listener_thread = thread::spawn(move || {
            let mut receiving_window: ReceiverWindow = Default::default();

            let mut buffer: Vec<u8> = vec![];

            loop {
                receiving_window
                    .shift_window()
                    .into_iter()
                    .for_each(|frame| {
                        // TODO: Handle incoming messages where they are in order, but k != 0 are not complete

                        buffer.extend_from_slice(frame.get_data());

                        let control_bits = ControlBits::from_bits(frame.get_control_bits())
                            .expect("Failed to parse control bits");

                        connection_events_sender_receiver
                            .send(ConnectionEventType::ReceivedFrame(frame))
                            .unwrap();

                        if control_bits.contains(ControlBits::EOM) {
                            let message: Vec<u8> = std::mem::take(&mut buffer);
                            connection_events_sender_receiver
                                .send(ConnectionEventType::ReceivedCompleteMessage(message))
                                .unwrap();
                        }
                    });

                let (size, buffer, addr) = listen_socket.receive().unwrap();

                let frame: Frame = buffer.into();

                let control_bits = ControlBits::from_bits(frame.get_control_bits())
                    .expect("Failed to parse control bits");

                let sequence_number = frame.get_sequence_number();

                if !control_bits.contains(ControlBits::ACK) {
                    let ack_frame = ConnectionManager::construct_ack_frame(sequence_number);
                    ack_socket.send(ack_frame.get_buffer()).unwrap();
                    connection_events_sender_receiver
                        .send(ConnectionEventType::SentAck(frame.clone()))
                        .unwrap();
                    receiving_window.handle_incoming_frame(frame);
                } else {
                    ack_sender.send(sequence_number).unwrap();
                    connection_events_sender_receiver
                        .send(ConnectionEventType::ReceivedAck(frame))
                        .unwrap();
                }

                println!("Received {} bytes from {}", size, addr);
            }
        });

        let transmitter_thread = thread::spawn(move || {
            let mut transmitter_window: TransmitterWindow = TransmitterWindow::new(
                ack_receiver,
                connection_events_sender_transmitter.clone(),
                sender_socket,
            );
            loop {
                let next_message: Vec<u8> = messages_to_send_receiver.recv().unwrap();

                transmitter_window.send_message(next_message);
                connection_events_sender_transmitter
                    .send(ConnectionEventType::SentMessage)
                    .unwrap();
            }
        });
        Ok((
            Self {
                listener_thread: Some(listener_thread),
                transmitter_thread: Some(transmitter_thread),
            },
            connection_events_receiver,
            messages_to_send,
        ))
    }

    fn construct_ack_frame(sequence_number: u32) -> Frame {
        let mut frame = Frame::default();
        frame.set_sequence_number(sequence_number);
        frame.set_control_bits(ControlBits::ACK.bits());
        frame
    }
}

impl Drop for ConnectionManager {
    fn drop(&mut self) {
        if let Some(x) = self.listener_thread.take() {
            x.join().unwrap();
        }
        if let Some(x) = self.transmitter_thread.take() {
            x.join().unwrap();
        }
    }
}
