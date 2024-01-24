use std::collections::HashMap;
use crate::transport_layer::frame::Frame;
use crate::transport_layer::receiver_window::ReceiverWindow;

#[derive(Default)]
pub struct ReceivingWindowManager {
    // If message channel id is 0, this channel will handle the message
    main_channel: ReceiverWindow,
    // If message channel id is not 0, find channel with that id if it exists, if not create it
    // Then message is completed, send it to application layer and remove channel. Send Remove channel message to other side
    fragment_channels: HashMap<u8, ReceiverWindow>,
}

impl ReceivingWindowManager {
    pub fn handle_incoming_frame(&mut self, frame: Frame) {
        if frame.get_channel_id() == 0 {
            self.main_channel.handle_incoming_frame(frame); // main channel
        } else {
            // fragment channel
            // TODO: Handle channel completion and removal
            let channel_id = frame.get_channel_id();
            let fragment_channel = self.fragment_channels.entry(channel_id).or_insert_with(|| ReceiverWindow::default());
            fragment_channel.handle_incoming_frame(frame);
        }
    }

    pub fn shift_all_windows(&mut self) -> HashMap<u8, Vec<Frame>> {
        let mut all_shifted_frames: HashMap<u8, Vec<Frame>> = Default::default();
        all_shifted_frames.insert(0, self.main_channel.shift_window());
        for (k, fragment_channel) in self.fragment_channels.iter_mut() {
            all_shifted_frames.insert(*k, fragment_channel.shift_window());
        }
        all_shifted_frames
    }
}