use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;

use log::{error, info};

use crate::transport_layer::constants::MAX_DATA_SIZE;
use crate::transport_layer::control_bits::ControlBits;
use crate::transport_layer::frame::Frame;
use crate::transport_layer::option::{FrameOption, OptionKind};
use crate::transport_layer::window::Window;

pub struct Receiver {
    /// The earliest sequence number that has not been received.
    earliest_not_received: Arc<AtomicU32>,

    /// Buffer to store received data.
    read_buffer: Arc<Mutex<Vec<u8>>>,

    /// Channel to send frame data to the application layer in correct order.
    message_sender: Sender<Vec<u8>>,
}

impl Receiver {
    pub fn new() -> (Self, std::sync::mpsc::Receiver<Vec<u8>>) {
        let (tx, rx) = channel();
        (Self {
            earliest_not_received: Arc::new(AtomicU32::new(0)),
            read_buffer: Arc::new(Mutex::new(vec![])),
            message_sender: tx,
        }, rx)
    }

    /// Listens for incoming segments from network.
    /// When a segment is received, it is stored in a buffer.
    /// When the End-of-Message control bit is received, the buffer is sent using incoming_messages channel.
    pub fn listen(&self, window: Arc<Window>) -> JoinHandle<()> {
        let read_buffer = Arc::clone(&self.read_buffer);
        let channel = self.message_sender.clone();
        let earliest_not_received_clone = self.earliest_not_received.clone();
        info!("Starting listening thread");

        thread::spawn(move || {

            let mut eom_sequence_number: Option<u32> = None;
            loop {
                let (_, buffer, _) = match window.receive() {
                    Ok(data) => data,
                    Err(error) => {
                        error!("Failed to receive data: {} trying again in one second", error);
                        thread::sleep(std::time::Duration::from_secs(1));
                        continue;
                    }
                };
                let frame: Frame = buffer.into();
                let sequence_number = frame.get_sequence_number();

                if let Some(eom_number) = eom_sequence_number {
                    let i = earliest_not_received_clone.load(Ordering::Relaxed);
                    info!("Received frame with sequence number {} while waiting for EOM with sequence number {}", sequence_number, eom_number);
                    if eom_number < i {
                        Receiver::construct_message(&read_buffer, &channel);
                        eom_sequence_number = None;
                        continue;
                    }
                }


                let control_bits = ControlBits::from_bits(frame.get_control_bits()).expect("Failed to parse control bits");

                if control_bits.contains(ControlBits::ACK) {
                    let acknowledgment_number = 0;
                    window.handle_acknowledgment(acknowledgment_number);
                    continue;
                }
                window.send_ack(sequence_number);

                Receiver::update_earliest_not_received(&earliest_not_received_clone, sequence_number);

                let mut buffer_offset = 1u32;

                if let Some(options) = frame.get_options() {
                    for option in options {
                        match option.kind {
                            OptionKind::BufferSize => {
                                Receiver::sync_read_buffer(&read_buffer, &option);
                            },
                            OptionKind::SegmentNumber => {
                                buffer_offset = u32::from_be_bytes(option.data.try_into().expect("Failed to convert buffer size to u32"));
                            }
                        }
                    }
                }

                let data = frame.get_data();

                Receiver::insert_data_into_buffer(&read_buffer, buffer_offset, data);
                info!("Received frame with sequence number {}", sequence_number);
                if control_bits.contains(ControlBits::EOM) {
                    eom_sequence_number = Some(sequence_number);
                    info!("Received End-of-Message");
                }
            }
        })
    }

    /// Constructs a message from the buffer and sends it to the application layer.
    fn construct_message(buffer: &Arc<Mutex<Vec<u8>>>, channel: &Sender<Vec<u8>>) {
        let buffer_guard = buffer.lock().unwrap();
        channel.send(buffer_guard.clone()).unwrap();
    }

    /// Inserts data into the buffer at the position derived from buffer offset.
    fn insert_data_into_buffer(buffer: &Arc<Mutex<Vec<u8>>>, buffer_offset: u32, data: &[u8]) {
        let mut buffer_guard = buffer.lock().unwrap();
        let buffer_shift = (buffer_offset as usize - 1) * MAX_DATA_SIZE;
        let data_upper_bound = buffer_shift + data.len();
        buffer_guard[buffer_shift..data_upper_bound].copy_from_slice(data);
    }

    /// Resizes the buffer to the size specified in the option.
    /// This operation is performed when the first frame is received.
    fn sync_read_buffer(read_buffer: &Arc<Mutex<Vec<u8>>>, option: &FrameOption) {
        let buffer_size = u32::from_be_bytes(option.data.try_into().expect("Failed to convert buffer size to u32"));
        let mut read_buffer_guard = read_buffer.lock().unwrap();
        info!("Resizing read buffer to {}", buffer_size);
        read_buffer_guard.resize(buffer_size as usize, 0);
    }

    /// Updates the earliest_not_received value.
    /// Earliest not received is updated only if the sequence number is the next expected sequence number.
    fn update_earliest_not_received(earliest_not_received: &Arc<AtomicU32>, sequence_number: u32) {
        let earliest_not_received_value = earliest_not_received.load(Ordering::Relaxed);
        if sequence_number == earliest_not_received_value + 1u32 {
            earliest_not_received.store(sequence_number + 1, Ordering::Relaxed);
        }
    }
}