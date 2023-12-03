use std::cmp::min;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use log::{error, info, warn};

use crate::constants::{MAX_DATA_SIZE, TIMEOUT, WINDOW_SIZE};
use crate::control_bits::ControlBits;
use crate::frame::Frame;
use crate::frame_status::FrameStatus;
use crate::option::{FrameOption, OptionKind};
use crate::socket::Socket;

pub struct Transmitter {
    /// The highest sequence number that has been acknowledged.
    /// Also marks the beginning of the window.
    smallest_acknowledged_sequence_number: RwLock<u32>,

    socket: Arc<Socket>,

    /// The status of each frame in the window.
    window_frame_statuses: Mutex<[FrameStatus; WINDOW_SIZE]>,
}


impl Transmitter {
    pub fn new(socket: Arc<Socket>) -> Self {
        Self {
            socket,
            smallest_acknowledged_sequence_number: RwLock::new(0),
            window_frame_statuses: Mutex::new([FrameStatus::NotSent; WINDOW_SIZE]),
        }
    }

    pub fn handle_acknowledgment(&self, acknowledgment_number: u32) {
        info!("Received ACK for sequence number {}", acknowledgment_number);
        let mut window_frame_statuses_guard = self.window_frame_statuses.lock().unwrap();
        let smallest_acknowledged_sequence_number_guard = self.smallest_acknowledged_sequence_number.read().unwrap();
        if acknowledgment_number <= *smallest_acknowledged_sequence_number_guard {
            info!("Received ACK for sequence number {} which is smaller than the smallest acknowledged sequence number {}", acknowledgment_number, *smallest_acknowledged_sequence_number_guard);
            return;
        }
        let index = (acknowledgment_number - (*smallest_acknowledged_sequence_number_guard + 1)) as usize;
        info!("Marking frame with sequence number {} as acknowledged", acknowledgment_number);
        window_frame_statuses_guard[index] = FrameStatus::Acknowledged;
    }

    pub fn shift_window(&self) {
        let mut window_frame_statuses = self.window_frame_statuses.lock().unwrap();
        match window_frame_statuses[0] {
            FrameStatus::Acknowledged => {}
            _ => return
        };
        let mut shift_amount = 1usize;
        for i in 1..WINDOW_SIZE {
            match window_frame_statuses[i] {
                FrameStatus::Acknowledged => {
                    shift_amount += 1;
                }
                _ => break
            }
        }
        for i in 0..WINDOW_SIZE {
            let shift_index = i + shift_amount;
            if shift_index >= WINDOW_SIZE {
                window_frame_statuses[i] = FrameStatus::NotSent;
            } else {
                window_frame_statuses[i] = window_frame_statuses[shift_index];
            }
        }
        info!("Shifting window by {}", shift_amount);
        let mut smallest_acknowledged_sequence_number_lock = self.smallest_acknowledged_sequence_number.write().unwrap();
        *smallest_acknowledged_sequence_number_lock += shift_amount as u32;
        info!("New smallest acknowledged sequence number: {}", smallest_acknowledged_sequence_number_lock);
    }

    pub fn send_ack(&self, sequence_number: u32) {
        for _ in 0..3 {
            let mut frame = Frame::default();
            frame.set_sequence_number(0);
            frame.set_acknowledgment_number(sequence_number);
            frame.set_control_bits(ControlBits::ACK.bits());
            match self.socket.send(frame.get_buffer()) {
                Ok(_) => {
                    info!("Sent ACK for sequence number {}", sequence_number);
                    break;
                }
                Err(e) => error!("Failed to send ACK: {} trying again", e)
            }
        }
    }

    pub fn send(&self, data_buffer: &[u8]) -> std::io::Result<usize> {
        let data_size = data_buffer.len() as u32;
        let segments = data_size as f32 / MAX_DATA_SIZE as f32;
        let segments = segments.ceil() as u32;

        let mut frame = Frame::default();


        let be_data_size = data_size.to_be_bytes();
        frame.set_options(&[FrameOption::new(OptionKind::BufferSize, &be_data_size)]);

        'sending: loop {
            self.shift_window();

            for i in 0..WINDOW_SIZE {
                let frame_status = { self.window_frame_statuses.lock().unwrap()[i] };

                let sequence_number = {
                    *self.smallest_acknowledged_sequence_number.read().unwrap() + i as u32 + 1
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
                info!("Sent frame with sequence number {}/{}", sequence_number, segments);
                self.socket.send(frame.get_buffer())?;
                { self.window_frame_statuses.lock().unwrap()[i] = FrameStatus::Sent(Instant::now()); }
                // Reset frame
                frame = Frame::default();
            }
        }
        Ok(0)
    }
}