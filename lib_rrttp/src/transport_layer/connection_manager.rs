use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::warn;

use crate::transport_layer::control_bits::ControlBits;
use crate::transport_layer::frame::Frame;
use crate::transport_layer::receiver_window::ReceiverWindow;
use crate::transport_layer::socket::Socket;
use crate::transport_layer::transmitter_window::TransmitterWindow;
use crate::transport_layer::window_manager::WindowManager;

pub enum ConnectionEventType {
    ReceivedComplete(Frame),
    // These fragments are always in order
    ReceivedFragment(Frame),
    ReceivedAck(Frame),
    SentAck(Frame),

    SentSingleComplete(Frame),
    SentFragment(Frame),
    AllFragmentsSent(ChannelId),
}

pub type SequenceNumber = u32;
pub type ChannelId = u32;

pub struct ConnectionManager {
    message_queue: Sender<Vec<u8>>,
    ack_channel_senders: Arc<Mutex<HashMap<ChannelId, Sender<SequenceNumber>>>>,
}

impl ConnectionManager {
    pub fn start(local_addr: &str) -> std::io::Result<(Self, Receiver<ConnectionEventType>)> {
        let (message_queue_sender, messages_to_send) = std::sync::mpsc::channel();

        let (connection_events_sender, connection_events_receiver) = std::sync::mpsc::channel();

        let listen_socket = Arc::new(Socket::bind(local_addr)?);

        let channel_senders_map = Arc::new(Mutex::new(HashMap::<ChannelId, Sender<SequenceNumber>>::new()));

        let ack_socket = listen_socket.clone();
        let sender_socket = listen_socket.clone();

        let connection_events_sender_receiver = connection_events_sender.clone();
        let connection_events_sender_transmitter = connection_events_sender.clone();

        let channel_senders_map_clone = channel_senders_map.clone();

        thread::spawn(move || {
            let mut receiving_window_manager: WindowManager<ReceiverWindow> = WindowManager::default();
            loop {
                receiving_window_manager.shift_all_windows().into_iter().for_each(|(channel_id, fragments_or_message)| {
                    // TODO: Handle incoming messages where they are in order, but k != 0 are not complete
                    for i in fragments_or_message.into_iter() {
                        let packet = match channel_id {
                            0 => ConnectionEventType::ReceivedComplete(i),
                            _ => ConnectionEventType::ReceivedFragment(i),
                        };
                        connection_events_sender_receiver.send(packet).unwrap();
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
                    connection_events_sender_receiver.send(ConnectionEventType::SentAck(frame.clone())).unwrap();
                    receiving_window_manager.handle_incoming_frame(frame);
                } else {
                    match channel_senders_map_clone.lock().unwrap().get(&channel_id) {
                        None => warn!("Received ack for unknown channel {}", channel_id),
                        Some(s) => s.send(frame.get_sequence_number()).unwrap()
                    }
                    connection_events_sender_receiver.send(ConnectionEventType::ReceivedAck(frame)).unwrap();
                }


                println!("Received {} bytes from {}", size, addr);
            }
        });

        let channel_senders_for_frag = channel_senders_map.clone();
        thread::spawn(move || {
            let mut sending_window_manager: WindowManager<TransmitterWindow> = WindowManager::default();
            let mut fragment_channel_id = 1u32;
            loop {
                sending_window_manager.shift_main_channel_window();
                let next_message: Vec<u8> = messages_to_send.recv().unwrap();

                let data_size = next_message.len();

                if sending_window_manager.needs_fragmentation(data_size) {
                    // TODO: spawn a thread to send the fragments
                    let fragment_channel_id_clone = fragment_channel_id;
                    fragment_channel_id += 1;

                    let connection_events_sender_frag = connection_events_sender_transmitter.clone();
                    let fragment_channel_senders = channel_senders_for_frag.clone();

                    let sender_socket_clone = sender_socket.clone();
                    thread::spawn(move || {
                        let channel_id = fragment_channel_id_clone;
                        let mut channel = TransmitterWindow::default();
                        let (sender, receiver) = std::sync::mpsc::channel();
                        {
                            fragment_channel_senders.lock().unwrap().insert(channel_id, sender);
                        }

                        if let Err(e) = channel.send_fragmented_data(channel_id, next_message, connection_events_sender_frag, receiver, sender_socket_clone) {
                            warn!("Failed to send fragmented data: {:?}", e);
                        }
                    });
                } else {
                    // TODO: send message without additional thread
                }
            }
        });
        Ok((Self {
            message_queue: message_queue_sender,
            ack_channel_senders: channel_senders_map,
        }, connection_events_receiver))
    }

    fn construct_ack_frame(channel_id: u32, sequence_number: u32) -> Frame {
        let mut frame = Frame::default();
        frame.set_sequence_number(sequence_number);
        frame.set_control_bits(ControlBits::ACK.bits());
        frame.set_channel_id(channel_id);
        frame
    }
}