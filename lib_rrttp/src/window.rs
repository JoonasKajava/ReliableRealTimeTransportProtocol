use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use log::{error, info, warn};

use crate::constants::{MAX_DATA_SIZE, TIMEOUT, WINDOW_SIZE};
use crate::control_bits::ControlBits;
use crate::frame::Frame;
use crate::frame_status::FrameStatus;
use crate::option::{FrameOption, OptionKind};
use crate::socket::Socket;

pub struct Window {
    window_size: u32,
    socket: Arc<Socket>,
    /// The highest sequence number that has been acknowledged.
    /// Also marks the beginning of the window.
    smallest_acknowledged_sequence_number: Arc<Mutex<u32>>,

    /// The highest sequence number that has been received.
    highest_received: Arc<Mutex<u32>>,

    /// The earliest sequence number that has not been received.
    earliest_not_received: Arc<Mutex<u32>>,

    window_frame_statuses: Arc<Mutex<[FrameStatus; WINDOW_SIZE]>>,

    /// Buffer to store received data.
    read_buffer: Arc<Mutex<Vec<u8>>>,

    /// Channel to send messages to the application layer.
    message_sender: Sender<Vec<u8>>,
    /// Channel to receive messages from the application layer.
    message_receiver: Receiver<Vec<u8>>,

}

impl Window {
    pub fn new(local_addr: &str, remote_addr: &str) -> std::io::Result<Self> {
        let socket = Socket::bind(local_addr)?;
        socket.connect(remote_addr)?;


        let (tx, rx) = channel();
        Ok(Self {
            window_size: WINDOW_SIZE as u32,
            socket: Arc::new(socket),
            smallest_acknowledged_sequence_number: Arc::new(Mutex::new(0)),
            highest_received: Arc::new(Mutex::new(0)),
            earliest_not_received: Arc::new(Mutex::new(0)),
            window_frame_statuses: Arc::new(Mutex::new([FrameStatus::NotSent; WINDOW_SIZE])),
            read_buffer: Arc::new(Mutex::new(Vec::new())),
            message_sender: tx,
            message_receiver: rx,
        })
    }

    pub fn incoming_messages(&self) -> &Receiver<Vec<u8>> {
        &self.message_receiver
    }

    fn handle_acknowledgment(smallest_acknowledged_sequence_number: &Arc<Mutex<u32>>, window_frame_statuses: &Arc<Mutex<[FrameStatus; WINDOW_SIZE]>>, acknowledgment_number: u32) {
        info!("Received ACK for sequence number {}", acknowledgment_number);
        let mut window_frame_statuses_guard = window_frame_statuses.lock().unwrap();
        let smallest_acknowledged_sequence_number_guard = smallest_acknowledged_sequence_number.lock().unwrap();
        let index = (acknowledgment_number - (*smallest_acknowledged_sequence_number_guard + 1)) as usize;
        if index >= WINDOW_SIZE {
            info!("Received ACK for sequence number {} which is outside of the window", acknowledgment_number);
            return;
        }
        window_frame_statuses_guard[index] = FrameStatus::Acknowledged;

    }

    pub fn listen(&mut self) -> JoinHandle<()> {
        let socket = Arc::clone(&self.socket);
        let highest_received = Arc::clone(&self.highest_received);
        let earliest_not_received = Arc::clone(&self.earliest_not_received);
        let smallest_acknowledged_sequence_number = Arc::clone(&self.smallest_acknowledged_sequence_number);
        let read_buffer = Arc::clone(&self.read_buffer);

        let window_frame_statuses = Arc::clone(&self.window_frame_statuses);

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
                    Window::handle_acknowledgment(&smallest_acknowledged_sequence_number, &window_frame_statuses, acknowledgment_number);
                    continue;
                }
                Window::send_ack(&socket, sequence_number);

                Window::update_earliest_not_received(&earliest_not_received, sequence_number);

                {
                    let mut highest_received_guard = highest_received.lock().unwrap();
                    if sequence_number > *highest_received_guard {
                        *highest_received_guard = sequence_number;
                    }
                }

                if let Some(options) = frame.get_options() {
                    for option in options {
                        match option.kind {
                            OptionKind::BufferSize => {
                                Window::sync_read_buffer(&read_buffer, &option);
                            }
                        }
                    }
                }

                let data = frame.get_data();

                Window::insert_data_into_buffer(&read_buffer, sequence_number, data);
                info!("Received frame with sequence number {}", sequence_number);
                if control_bits.contains(ControlBits::EOM) {
                    info!("Received End-of-Message");
                    Window::construct_message(&read_buffer, channel);
                    break;
                }
            }
        })
    }

    fn shift_window(&mut self) {
        let mut window_frame_statuses_guard = self.window_frame_statuses.lock().unwrap();
        match window_frame_statuses_guard[0] {
            FrameStatus::Acknowledged => {}
            _ => return
        };
        let mut shift_amount= 1usize;
        let window_size = self.window_size as usize;
        for i in 1..window_size {
            match window_frame_statuses_guard[i] {
                FrameStatus::Acknowledged => {
                    shift_amount += 1;
                }
                _ => break
            }
        }
        for i in 0..window_size {
            let shift_index = i + shift_amount;
            if shift_index >= window_size {
                window_frame_statuses_guard[i] = FrameStatus::NotSent;
            }else {
                window_frame_statuses_guard[i] = window_frame_statuses_guard[shift_index];
            }
        }
        let mut smallest_acknowledged_sequence_number_guard = self.smallest_acknowledged_sequence_number.lock().unwrap();
        info!("Shifting window by {}", shift_amount);
        *smallest_acknowledged_sequence_number_guard += shift_amount as u32;
        info!("New smallest acknowledged sequence number: {}", *smallest_acknowledged_sequence_number_guard);
    }

    fn insert_data_into_buffer(buffer: &Arc<Mutex<Vec<u8>>>, sequence_number: u32, data: &[u8]) {
        let mut buffer_guard = buffer.lock().unwrap();
        let buffer_shift = (sequence_number as usize - 1) * MAX_DATA_SIZE;
        let data_upper_bound = buffer_shift + data.len();
        buffer_guard[buffer_shift..data_upper_bound].copy_from_slice(data);
    }

    fn construct_message(buffer: &Arc<Mutex<Vec<u8>>>, channel: Sender<Vec<u8>>) {
        let buffer_guard = buffer.lock().unwrap();
        channel.send(buffer_guard.clone()).unwrap();
    }


    fn update_earliest_not_received(earliest_not_received: &Arc<Mutex<u32>>, sequence_number: u32) {
        let mut earliest_not_received_guard = earliest_not_received.lock().unwrap();
        if sequence_number == *earliest_not_received_guard + 1u32 {
            *earliest_not_received_guard = sequence_number + 1;
        }
    }

    fn sync_read_buffer(read_buffer: &Arc<Mutex<Vec<u8>>>, option: &FrameOption) {
        let buffer_size = u32::from_be_bytes(option.data.try_into().expect("Failed to convert buffer size to u32"));
        let mut read_buffer_guard = read_buffer.lock().unwrap();
        info!("Resizing read buffer to {}", buffer_size);
        read_buffer_guard.resize(buffer_size as usize, 0);
    }

    pub fn send_ack(socket: &Socket, sequence_number: u32) {
        for _ in 0..3 {
            let mut frame = Frame::default();
            frame.set_sequence_number(0);
            frame.set_acknowledgment_number(sequence_number);
            frame.set_control_bits(ControlBits::ACK.bits());
            match socket.send(frame.get_buffer()) {
                Ok(_) => {
                    info!("Sent ACK for sequence number {}", sequence_number);
                    break;
                }
                Err(e) => error!("Failed to send ACK: {} trying again", e)
            }
        }
    }

    pub fn send(&mut self, data_buffer: &[u8]) -> std::io::Result<usize> {
        let data_size = data_buffer.len() as u32;
        let segments = data_size as f32 / MAX_DATA_SIZE as f32;
        let segments = segments.ceil() as u32;

        let mut frame = Frame::default();


        let be_data_size = data_size.to_be_bytes();
        frame.set_options(&[FrameOption::new(OptionKind::BufferSize, &be_data_size)]);

        'sending: loop {

            self.shift_window();

            for i in 0..self.window_size {
                let frame_status = {
                    self.window_frame_statuses.lock().unwrap()[i as usize]
                };

                let sequence_number = {
                    *self.smallest_acknowledged_sequence_number.lock().unwrap() + i + 1
                };
                match frame_status {
                    FrameStatus::Sent(timestamp) if timestamp.elapsed().as_millis() <= TIMEOUT => continue,
                    FrameStatus::Sent(_) => warn!("Frame with sequence number {} timed out", sequence_number),
                    FrameStatus::Acknowledged => continue,
                    _ => {}
                }


                if sequence_number > segments {
                    info!("Finished sending");
                    break 'sending;
                }

                // Construct frame

                frame.set_sequence_number(sequence_number);


                frame.set_acknowledgment_number(0);

                if sequence_number == segments {
                    frame.set_control_bits(ControlBits::EOM.bits());
                } else {
                    frame.set_control_bits(0);
                }
                // Set Data

                let buffer_shift = (sequence_number - 1) * MAX_DATA_SIZE as u32;

                let buffer_left = data_size - buffer_shift;

                let data_lower_bound = buffer_shift as usize;
                let data_upper_bound = (buffer_shift + min(buffer_left, MAX_DATA_SIZE as u32)) as usize;

                let data_segment = &data_buffer[data_lower_bound..data_upper_bound];
                frame.set_data(data_segment);


                // Send frame
                info!("Sent frame with sequence number {}", sequence_number);
                self.socket.send(frame.get_buffer())?;
                self.window_frame_statuses.lock().unwrap()[i as usize] = FrameStatus::Sent(Instant::now());
                // Reset frame
                frame = Frame::default();
            }
        }
        Ok(0)
    }
}