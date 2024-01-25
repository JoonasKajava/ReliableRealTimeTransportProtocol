use std::sync::mpsc::Sender;
use crate::transport_layer::connection_manager::{ConnectionEventType, SequenceNumber};
use crate::transport_layer::frame::Frame;
use crate::transport_layer::frame_status::FrameStatus;
use crate::transport_layer::window::NewWindow;

#[derive(Default)]
pub struct TransmitterWindow {
    inner_window: NewWindow,
    window_status: Vec<FrameStatus>
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

    pub fn handle_outgoing_data(&mut self, channel_id: u32, data: Vec<u8>, event_sender: Sender<ConnectionEventType>, ack_receiver: std::sync::mpsc::Receiver<SequenceNumber>) {
        loop {

            for i in ack_receiver.try_iter() {
                let index = self.inner_window.get_window_index(i);
                self.window_status[index] = FrameStatus::Acknowledged;
            }

            self.shift_window();
            let test = event_sender.send(ConnectionEventType::ReceivedAck(Frame::default()));
        }
    }



}