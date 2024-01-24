use std::collections::HashMap;
use crate::transport_layer::constants::MAX_DATA_SIZE;
use crate::transport_layer::frame::Frame;
use crate::transport_layer::receiver_window::ReceiverWindow;
use crate::transport_layer::transmitter_window::TransmitterWindow;


#[derive(Default)]
pub struct WindowManager<TWindow> {
    main_channel: TWindow,
    fragment_channels: HashMap<u32, TWindow>,
}

impl WindowManager<ReceiverWindow> {
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
    pub fn shift_all_windows(&mut self) -> HashMap<u32, Vec<Frame>> {
        let mut all_shifted_frames: HashMap<u32, Vec<Frame>> = Default::default();
        all_shifted_frames.insert(0, self.main_channel.shift_window());
        for (k, fragment_channel) in self.fragment_channels.iter_mut() {
            all_shifted_frames.insert(*k, fragment_channel.shift_window());
        }
        all_shifted_frames
    }
}

impl WindowManager<TransmitterWindow> {
    pub fn shift_main_channel_window(&mut self) {
        self.main_channel.shift_window();
    }

    pub fn needs_fragmentation(&self, data_size: usize) -> bool {
        data_size > MAX_DATA_SIZE
    }
}