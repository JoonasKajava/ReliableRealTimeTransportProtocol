use std::cmp::min;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use log::warn;
use crate::application_layer::connection_manager::{ConnectionEventType, SequenceNumber};

use crate::transport_layer::constants::{MAX_DATA_SIZE, TIMEOUT};
use crate::transport_layer::control_bits::ControlBits;
use crate::transport_layer::frame::Frame;
use crate::transport_layer::frame_status::FrameStatus;
use crate::transport_layer::socket::Socket;
use crate::transport_layer::window::NewWindow;

pub struct TransmitterWindow {
    inner_window: NewWindow,
    window_status: Vec<FrameStatus>,

    socket: Arc<Socket>,

    ack_receiver: Receiver<SequenceNumber>,

    events_sender: Sender<ConnectionEventType>,
}

impl TransmitterWindow {
    pub fn new(
        ack_receiver: Receiver<SequenceNumber>,
        events_sender: Sender<ConnectionEventType>,
        socket: Arc<Socket>,
    ) -> Self {
        Self {
            inner_window: Default::default(),
            window_status: vec![],
            ack_receiver,
            events_sender,
            socket,
        }
    }


    pub fn set_window_size(&mut self, size: u32) {
        self.inner_window.set_window_size(size);
    }

    pub fn get_window_size(&self) -> u32 {
        self.inner_window.get_window_size()
    }

    pub fn get_window_left_edge(&self) -> u32 {
        self.inner_window.get_window_left_edge()
    }

    pub fn handle_acknowledgment(&mut self, acknowledgment_number: SequenceNumber) {
        if !self.is_within_window(acknowledgment_number) {
            return;
        }
        let index = self.inner_window.get_window_index(acknowledgment_number);
        self.window_status[index] = FrameStatus::Acknowledged;
        self.inner_window.update_frame_status(index);
    }

    pub fn is_within_window(&self, sequence_number: u32) -> bool {
        self.inner_window.is_within_window(sequence_number)
    }

    pub fn shift_window(&mut self) -> usize {
        let shift_amount = self.inner_window.shift_window();
        self.window_status.drain(0..shift_amount);
        shift_amount
    }

    pub fn send_message(&mut self, next_message: Vec<u8>) {
        let data_size = next_message.len();
        let fragments = (data_size as f64 / MAX_DATA_SIZE as f64).ceil() as u32;

        let left_edge_goal = self.get_window_left_edge() + fragments; // TODO: Maybe better way to do this

        let mut frame: Frame = Default::default();

        loop {
            let sequence_numbers: Vec<SequenceNumber> = self.ack_receiver.try_iter().collect();

            for i in sequence_numbers {
                self.handle_acknowledgment(i);
            }
            self.shift_window();

            let window_left_edge = self.get_window_left_edge();

            if window_left_edge >= left_edge_goal {
                // TODO: Maybe better message?
                self.events_sender.send(ConnectionEventType::SentMessage).unwrap();
                break;
            }

            let window_size = self.get_window_size();
            let window_range = min(window_size, left_edge_goal - window_left_edge) as usize;

            for i in 0..window_range {
                let frame_status = self.window_status[i];

                let sequence_number = window_left_edge + i as u32 + 1;

                match frame_status {
                    // TODO: Implement smarter timeout
                    FrameStatus::Sent(timestamp) if timestamp.elapsed().as_millis() <= TIMEOUT => continue,
                    FrameStatus::Sent(_) => warn!("Frame with sequence number {} timed out", sequence_number),
                    FrameStatus::Acknowledged => continue,
                    _ => {} // Not sent or timed out
                }

                frame.set_sequence_number(sequence_number);


                if sequence_number == left_edge_goal {
                    frame.set_control_bits(ControlBits::EOM.bits());
                }

                let buffer_shift = (sequence_number as usize - 1usize) * MAX_DATA_SIZE;
                let buffer_left = data_size - buffer_shift;

                debug_assert!(buffer_left > 0);

                let data_lower_bound = buffer_shift;
                let data_upper_bound = buffer_shift + min(buffer_left, MAX_DATA_SIZE);

                let data_slice = &next_message[data_lower_bound..data_upper_bound];

                frame.set_data(data_slice);

                match self.socket.send(frame.get_buffer()) {
                    Ok(_) => {
                        self.window_status[i] = FrameStatus::Sent(std::time::Instant::now());
                        self.events_sender.send(ConnectionEventType::SentFrame(frame.clone())).unwrap();
                    }
                    Err(e) => warn!("Failed to send frame: {}", e)
                }

                frame = Frame::default();
            }
        }
    }
}