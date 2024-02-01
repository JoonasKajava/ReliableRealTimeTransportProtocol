use std::cmp::min;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::warn;
use crate::transport_layer::connection_manager::{ChannelId, ConnectionEventType, SequenceNumber};
use crate::transport_layer::constants::{MAX_DATA_SIZE, TIMEOUT};
use crate::transport_layer::frame::Frame;
use crate::transport_layer::frame_status::FrameStatus;
use crate::transport_layer::socket::Socket;
use crate::transport_layer::window::NewWindow;

#[derive(Default)]
pub struct TransmitterWindow {
    inner_window: NewWindow,
    window_status: Vec<FrameStatus>,
}

impl TransmitterWindow {
    pub fn set_window_size(&mut self, size: u32) {
        self.inner_window.set_window_size(size);
    }

    pub fn is_within_window(&self, sequence_number: u32) -> bool {
        self.inner_window.is_within_window(sequence_number)
    }

    pub fn shift_window(&mut self) -> usize {
        let shift_amount = self.inner_window.shift_window();
        self.window_status.drain(0..shift_amount);
        shift_amount
    }

    pub fn send_fragmented_data(&mut self, channel_id: ChannelId, data: Vec<u8>, event_sender: Sender<ConnectionEventType>, ack_receiver: std::sync::mpsc::Receiver<SequenceNumber>, socket: Arc<Socket>) -> Result<(), FragmentSendingError> {
        if channel_id < 1 {
            return Err(FragmentSendingError::IncorrectChannelId("Channel id 0 is reserved for complete messages"));
        }
        let data_size = data.len();

        if data_size > u32::MAX as usize {
            return Err(FragmentSendingError::DataTooLarge("Data is too large to send"));
        }

        let fragments = (data_size as f64 / MAX_DATA_SIZE as f64).ceil() as u32;

        let mut frame = Frame::default();

        loop {
            for i in ack_receiver.try_iter() {
                let index = self.inner_window.get_window_index(i);
                self.window_status[index] = FrameStatus::Acknowledged;
                self.inner_window.update_frame_status(index);
            }
            self.shift_window();

            let window_left_edge = self.inner_window.get_window_left_edge();

            if window_left_edge == fragments {
                event_sender.send(ConnectionEventType::AllFragmentsSent(channel_id)).unwrap();
                break;
            }

            let window_size = self.inner_window.get_window_size() as usize;
            for i in 0..window_size {
                let frame_status = self.window_status[i];

                let sequence_number = window_left_edge + i as u32 + 1;

                match frame_status {
                    FrameStatus::Sent(timestamp) if timestamp.elapsed().as_millis() <= TIMEOUT => continue,
                    FrameStatus::Sent(_) => warn!("Frame with sequence number {} timed out", sequence_number),
                    FrameStatus::Acknowledged => continue,
                    _ => {} // Not sent or timed out
                }


                frame.set_sequence_number(sequence_number);

                // TODO: Set control bits

                let buffer_shift = (sequence_number as usize - 1usize) * MAX_DATA_SIZE;
                let buffer_left = data_size - buffer_shift;

                if buffer_left == 0 {
                    break;
                }

                let data_lower_bound = buffer_shift;
                let data_upper_bound = buffer_shift + min(buffer_left, MAX_DATA_SIZE);

                let data_slice = &data[data_lower_bound..data_upper_bound];

                frame.set_data(data_slice);

                match socket.send(frame.get_buffer()) {
                    Ok(_) => {
                        self.window_status[i] = FrameStatus::Sent(std::time::Instant::now());
                        event_sender.send(ConnectionEventType::SentFragment(frame.clone())).unwrap();
                    }
                    Err(e) => return Err(FragmentSendingError::FailedToSendFrame(e))
                }

                frame = Frame::default();
            }
        }

        Ok(())
    }
}




#[derive(Debug)]
pub enum FragmentSendingError {
    IncorrectChannelId(&'static str),
    DataTooLarge(&'static str),
    FailedToSendFrame(std::io::Error),
}