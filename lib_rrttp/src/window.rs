use std::str::from_utf8;
use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use log::{error, info};

use crate::constants::{MAX_DATA_SIZE, WINDOW_SIZE};
use crate::control_bits::ControlBits;
use crate::frame::Frame;
use crate::socket::Socket;

pub struct Window {
    window_size: u32,
    socket: Arc<Socket>,
    /// The highest sequence number that has been acknowledged.
    /// Also marks the beginning of the window.
    highest_acknowledged: Arc<Mutex<u32>>,

    /// The highest sequence number that has been received.
    highest_received: Arc<Mutex<u32>>,
}

impl Window {
    pub fn new(local_addr: &str, remote_addr: &str) -> std::io::Result<Self> {
        let socket = Socket::bind(local_addr)?;
        socket.connect(remote_addr)?;

        Ok(Self {
            window_size: WINDOW_SIZE as u32,
            socket: Arc::new(socket),
            highest_acknowledged: Arc::new(Mutex::new(0)),
            highest_received: Arc::new(Mutex::new(0))
        })
    }

    pub fn read(&mut self) -> JoinHandle<()> {
        let socket = Arc::clone(&self.socket);
        let highest_acknowledged = Arc::clone(&self.highest_acknowledged);
        let highest_received = Arc::clone(&self.highest_received);
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
                    {
                        let mut highest_acknowledged_guard = highest_acknowledged.lock().unwrap();
                        if acknowledgment_number > *highest_acknowledged_guard {
                            info!("Received ACK for sequence number {}", acknowledgment_number);
                            *highest_acknowledged_guard = acknowledgment_number;
                        }
                    }
                    continue;
                }
                if let Err(e) = Window::send_ack(&socket, sequence_number) {
                    error!("Failed to send ACK: {}", e);
                }
                {
                    let mut highest_received_guard = highest_received.lock().unwrap();
                    if sequence_number > *highest_received_guard {
                        info!("Received frame with sequence number {}", sequence_number);
                        *highest_received_guard = sequence_number;
                    }
                }
                let data = frame.get_data();
                info!("Received frame with sequence number {} data: {}", sequence_number, from_utf8(data).unwrap());
            }
        })
    }

    pub fn send_ack(socket: &Socket, sequence_number: u32) -> std::io::Result<usize> {
        let mut frame = Frame::default();
        frame.set_sequence_number(0);
        frame.set_acknowledgment_number(sequence_number);
        frame.set_control_bits(ControlBits::ACK.bits());
        socket.send(frame.get_buffer())
    }

    pub fn send(&mut self, data_buffer: &[u8]) -> std::io::Result<usize> {
        let segments = data_buffer.len() as f32 / MAX_DATA_SIZE as f32;
        let segments = segments.ceil() as u32;

        for i in 0..self.window_size {
            let sequence_number = {
                *self.highest_acknowledged.lock().unwrap() + i + 1
            };

            if sequence_number > segments {
                break;
            }

            // Construct frame
            let mut frame = Frame::default();
            frame.set_sequence_number(sequence_number);


            frame.set_acknowledgment_number(0);
            frame.set_control_bits(0);
            // Set Data

            let buffer_shift = (sequence_number - 1) * MAX_DATA_SIZE as u32;

            let buffer_left = data_buffer.len() as u32 - buffer_shift;

            let data_lower_bound = buffer_shift as usize;
            let data_upper_bound = (buffer_shift + min(buffer_left, MAX_DATA_SIZE as u32)) as usize;

            let data_segment = &data_buffer[data_lower_bound..data_upper_bound];
            frame.set_data(data_segment);


            // Send frame
            info!("Sent frame with sequence number {}", sequence_number);
            self.socket.send(frame.get_buffer())?;
        }
        Ok(0)
    }
}