use crate::transport_layer::control_bits::ControlBits;
use log::info;
use std::sync::mpsc::{channel, sync_channel, Receiver, SendError, SyncSender};
use std::sync::Arc;
use std::thread;

use crate::transport_layer::frame::Frame;
use crate::transport_layer::receiver_window::ReceiverWindow;
use crate::transport_layer::socket::{SocketAbstraction, SocketInterface};
use crate::transport_layer::transmitter_window::TransmitterWindow;

#[derive(Debug)]
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
    socket: Arc<SocketAbstraction>,
    message_sender: SyncSender<Vec<u8>>,
}

impl ConnectionManager {
    pub fn start(local_addr: &str) -> std::io::Result<ConnectionManagerInterface> {
        let (connection_events_sender, connection_events_receiver) = channel();

        let (messages_to_send, messages_to_send_receiver) = sync_channel(100);

        let SocketInterface {
            ack_receiver,
            data_receiver,
            socket,
        } = SocketAbstraction::bind(local_addr)?;

        let connection_events_sender_receiver = connection_events_sender.clone();
        let connection_events_sender_transmitter = connection_events_sender.clone();

        let socket_for_ack = socket.clone();
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

                let frame: Frame = data_receiver.recv().unwrap();
                info!("Received frame: {:?}", frame);
                let sequence_number = frame.get_sequence_number();

                socket_for_ack.send_ack(sequence_number).unwrap();
                connection_events_sender_receiver
                    .send(ConnectionEventType::SentAck(frame.clone()))
                    .unwrap();
                receiving_window.handle_incoming_frame(frame);
            }
        });

        let socket_for_transmitter = socket.clone();
        let transmitter_thread = thread::spawn(move || {
            let mut transmitter_window: TransmitterWindow = TransmitterWindow::new(
                connection_events_sender_transmitter.clone(),
                socket_for_transmitter.clone(),
                ack_receiver,
            );
            loop {
                for i in messages_to_send_receiver.try_iter() {
                    transmitter_window.append_to_queue(i);
                }
                transmitter_window.send_from_queue();
            }
        });

        let connection_manager = Self {
            listener_thread: Some(listener_thread),
            transmitter_thread: Some(transmitter_thread),
            socket,
            message_sender: messages_to_send.clone(),
        };

        Ok(ConnectionManagerInterface {
            connection_manager,
            connection_events: connection_events_receiver,
            message_sender: messages_to_send,
        })
    }

    pub fn connect(&self, addr: &str) -> std::io::Result<()> {
        self.socket.connect(addr)
    }

    pub fn send(&self, message: Vec<u8>) -> Result<(), SendError<Vec<u8>>> {
        self.message_sender.send(message)
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

impl ConnectionManager {
    pub fn socket(&self) -> &SocketAbstraction {
        &self.socket
    }
}

pub struct ConnectionManagerInterface {
    pub connection_manager: ConnectionManager,
    pub connection_events: Receiver<ConnectionEventType>,
    pub message_sender: SyncSender<Vec<u8>>,
}
