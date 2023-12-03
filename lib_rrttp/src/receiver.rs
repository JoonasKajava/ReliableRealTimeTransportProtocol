use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

use log::{error, info};

use crate::constants::MAX_DATA_SIZE;
use crate::control_bits::ControlBits;
use crate::frame::Frame;
use crate::option::{FrameOption, OptionKind};
use crate::socket::Socket;
use crate::transmitter::Transmitter;

pub struct Receiver {
    socket: Arc<Socket>,

    transmitter: Arc<Transmitter>,
    /// The earliest sequence number that has not been received.
    earliest_not_received: Arc<RwLock<u32>>,

    /// Buffer to store received data.
    read_buffer: Arc<Mutex<Vec<u8>>>,

    /// Channel to send messages to the application layer.
    message_sender: Sender<Vec<u8>>,
    /// Channel to receive messages from the application layer.
    message_receiver: std::sync::mpsc::Receiver<Vec<u8>>,
}

impl Receiver {
    pub fn incoming_messages(&self) -> &std::sync::mpsc::Receiver<Vec<u8>> {
        &self.message_receiver
    }

    pub fn new(socket: Arc<Socket>, transmitter: Arc<Transmitter>) -> Self {
        let (tx, rx) = channel();
        Self {
            socket,
            transmitter,
            earliest_not_received: Arc::new(RwLock::new(0)),
            read_buffer: Arc::new(Mutex::new(vec![])),
            message_sender: tx,
            message_receiver: rx,
        }
    }
    pub fn listen(&self) -> JoinHandle<()> {
        let socket = Arc::clone(&self.socket);
        let earliest_not_received = Arc::clone(&self.earliest_not_received);
        let read_buffer = Arc::clone(&self.read_buffer);


        let transmitter = Arc::clone(&self.transmitter);

        let channel = self.message_sender.clone();
        thread::spawn(move || {
            loop {
                let (_, buffer, _) = match socket.receive() {
                    Ok(data) => data,
                    Err(error) => {
                        error!("Failed to receive data: {} trying again in one second", error);
                        thread::sleep(std::time::Duration::from_secs(1));
                        continue;
                    }
                };
                let frame: Frame = buffer.into();
                let control_bits = ControlBits::from_bits(frame.get_control_bits()).expect("Failed to parse control bits");

                let sequence_number = frame.get_sequence_number();
                if control_bits.contains(ControlBits::ACK) {
                    let acknowledgment_number = frame.get_acknowledgment_number();
                    transmitter.handle_acknowledgment(acknowledgment_number);
                    continue;
                }
                transmitter.send_ack(sequence_number);

                Receiver::update_earliest_not_received(&earliest_not_received, sequence_number);

                if let Some(options) = frame.get_options() {
                    for option in options {
                        match option.kind {
                            OptionKind::BufferSize => {
                                Receiver::sync_read_buffer(&read_buffer, &option);
                            }
                        }
                    }
                }

                let data = frame.get_data();

                Receiver::insert_data_into_buffer(&read_buffer, sequence_number, data);
                info!("Received frame with sequence number {}", sequence_number);
                if control_bits.contains(ControlBits::EOM) {
                    info!("Received End-of-Message");
                    Receiver::construct_message(&read_buffer, channel);
                    break;
                }
            }
        })
    }

    fn construct_message(buffer: &Arc<Mutex<Vec<u8>>>, channel: Sender<Vec<u8>>) {
        let buffer_guard = buffer.lock().unwrap();
        channel.send(buffer_guard.clone()).unwrap();
    }

    fn insert_data_into_buffer(buffer: &Arc<Mutex<Vec<u8>>>, sequence_number: u32, data: &[u8]) {
        let mut buffer_guard = buffer.lock().unwrap();
        let buffer_shift = (sequence_number as usize - 1) * MAX_DATA_SIZE;
        let data_upper_bound = buffer_shift + data.len();
        buffer_guard[buffer_shift..data_upper_bound].copy_from_slice(data);
    }

    fn sync_read_buffer(read_buffer: &Arc<Mutex<Vec<u8>>>, option: &FrameOption) {
        let buffer_size = u32::from_be_bytes(option.data.try_into().expect("Failed to convert buffer size to u32"));
        let mut read_buffer_guard = read_buffer.lock().unwrap();
        info!("Resizing read buffer to {}", buffer_size);
        read_buffer_guard.resize(buffer_size as usize, 0);
    }

    fn update_earliest_not_received(earliest_not_received: &Arc<RwLock<u32>>, sequence_number: u32) {
        let mut earliest_not_received_guard = earliest_not_received.write().unwrap();
        if sequence_number == *earliest_not_received_guard + 1u32 {
            *earliest_not_received_guard = sequence_number + 1;
        }
    }
}