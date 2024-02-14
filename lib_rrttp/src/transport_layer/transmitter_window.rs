use std::cmp::min;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use log::kv::Key;
use log::{info, warn};

use crate::application_layer::connection_manager::{ConnectionEventType, SequenceNumber};
use crate::transport_layer::constants::{MAX_DATA_SIZE, TIMEOUT};
use crate::transport_layer::control_bits::ControlBits;
use crate::transport_layer::frame::Frame;
use crate::transport_layer::frame_status::FrameStatus;
use crate::transport_layer::socket::SocketAbstraction;
use crate::transport_layer::window::NewWindow;

pub struct QueueFrame {
    frame: Frame,
    status: FrameStatus,
}

pub struct TransmitterWindow {
    inner_window: NewWindow,

    socket: Arc<SocketAbstraction>,

    events_sender: Sender<ConnectionEventType>,

    ack_receiver: Receiver<u32>,
    // For new implementation
    data_queue: Vec<Option<QueueFrame>>,
}

impl TransmitterWindow {
    pub fn new(
        events_sender: Sender<ConnectionEventType>,
        socket: Arc<SocketAbstraction>,
        receiver: Receiver<u32>,
    ) -> Self {
        Self {
            inner_window: Default::default(),
            events_sender,
            socket,
            data_queue: vec![],
            ack_receiver: receiver,
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
        if let Some(index) = self.inner_window.get_window_index(acknowledgment_number) {
            if let Some(frame_from_queue) = self.data_queue.get_mut(index).unwrap_or(&mut None) {
                frame_from_queue.status = FrameStatus::Acknowledged;

                self.inner_window.update_frame_status(index);
            }
        }
    }

    pub fn is_within_window(&self, sequence_number: u32) -> bool {
        self.inner_window.is_within_window(sequence_number)
    }

    pub fn shift_window(&mut self) -> usize {
        let shift_amount = self.inner_window.shift_window();
        self.data_queue.drain(0..shift_amount);
        shift_amount
    }

    pub fn append_to_queue(&mut self, data: impl Into<Vec<u8>>) {
        info!("Appending data to queue");
        let data_bytes = data.into();
        let data_size = data_bytes.len();
        let fragments = (data_size as f64 / MAX_DATA_SIZE as f64).ceil() as u32;

        for i in 0..fragments {
            let buffer_shift = (i as usize) * MAX_DATA_SIZE;
            // TODO: There is something fishy here
            // Null bytes are being appended to the end of the data
            // and EOM is not being set correctly
            let buffer_left = data_size - buffer_shift;

            let data_lower_bound = buffer_shift;
            let data_upper_bound = buffer_shift + min(buffer_left, MAX_DATA_SIZE);

            let data_slice = &data_bytes[data_lower_bound..data_upper_bound];

            let mut frame = Frame::default();
            frame.set_data(data_slice);

            if i == fragments - 1 {
                frame.set_control_bits(ControlBits::EOM.bits());
            }
            self.data_queue.push(Some(QueueFrame {
                frame,
                status: FrameStatus::NotSent,
            }));
        }
    }

    pub fn send_from_queue(&mut self) {
        let window_size = self.get_window_size() as usize;
        let window_left_edge = self.get_window_left_edge();

        while let Ok(acknowledgment_number) = self.ack_receiver.try_recv() {
            info!("Received acknowledgment: {}", acknowledgment_number);
            self.handle_acknowledgment(acknowledgment_number);
        }

        for (i, queue_frame_option) in self.data_queue.iter_mut().take(window_size).enumerate() {
            let queue_frame = match queue_frame_option {
                Some(frame) => frame,
                None => continue,
            };
            let sequence_number = window_left_edge + i as u32 + 1;
            let frame = &mut queue_frame.frame;
            match queue_frame.status {
                // TODO: Implement smarter timeout
                FrameStatus::Sent(timestamp) if timestamp.elapsed().as_millis() <= TIMEOUT => {
                    continue;
                }
                FrameStatus::Sent(_) => {
                    warn!("Frame with sequence number {} timed out", sequence_number)
                }
                FrameStatus::Acknowledged => continue,
                _ => {} // Not sent or timed out
            }

            frame.set_sequence_number(sequence_number);

            match self.socket.send(frame.get_buffer()) {
                Ok(_) => {
                    queue_frame.status = FrameStatus::Sent(std::time::Instant::now());
                    self.events_sender
                        .send(ConnectionEventType::SentFrame(frame.clone()))
                        .unwrap();
                }
                Err(e) => warn!("Failed to send frame: {}", e),
            }
        }

        self.shift_window();
    }
}
