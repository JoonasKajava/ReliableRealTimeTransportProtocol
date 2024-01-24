use crate::transport_layer::frame::Frame;
use crate::transport_layer::window::NewWindow;

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

    pub fn shift_window(&mut self) -> Vec<Frame> {
        let shift_amount = self.inner_window.shift_window();
        let shifted_frames = self.buffer.drain(0..shift_amount);

        shifted_frames.into_iter().filter_map(|e| e).collect()
    }

    pub fn handle_incoming_frame(&mut self, frame: Frame) {
        if !self.is_within_window(frame.get_sequence_number()) {
            return;
        }
        let index = (frame.get_sequence_number() - self.inner_window.get_left_edge()) as usize;
        self.buffer[index] = Some(frame);
        self.inner_window.update_frame_status(index)
    }
}