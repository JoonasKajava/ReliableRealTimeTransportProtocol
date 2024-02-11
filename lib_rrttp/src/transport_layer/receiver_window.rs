use crate::transport_layer::frame::Frame;
use crate::transport_layer::window::NewWindow;
use log::kv::Key;
use std::num::FpCategory::Zero;

#[derive(Default)]
pub struct ReceiverWindow {
    inner_window: NewWindow,
    buffer: Vec<Option<Frame>>,
}

impl ReceiverWindow {
    pub fn set_window_size(&mut self, size: u32) {
        self.inner_window.set_window_size(size);
        self.buffer.resize(size as usize, None);
    }

    pub fn is_within_window(&self, sequence_number: u32) -> bool {
        self.inner_window.is_within_window(sequence_number)
    }

    /// Shifts the window and returns the shifted frames.
    ///
    /// # Returns
    ///
    /// A vector containing the shifted frames in order.
    ///
    pub fn shift_window(&mut self) -> Vec<Frame> {
        let shift_amount = self.inner_window.shift_window();
        let shifted_frames = self.buffer.drain(0..shift_amount);
        let result = shifted_frames.into_iter().flatten().collect();
        self.buffer.shrink_to_fit();
        result
    }

    pub fn handle_incoming_frame(&mut self, frame: Frame) {
        if !self.is_within_window(frame.get_sequence_number()) {
            return;
        }
        let index = self
            .inner_window
            .get_window_index(frame.get_sequence_number());
        // TODO: There could be a better way to handle this
        if index >= self.buffer.len() {
            self.buffer.resize(index + 1, None);
        }
        self.buffer[index] = Some(frame);
        self.inner_window.update_frame_status(index)
    }
}
