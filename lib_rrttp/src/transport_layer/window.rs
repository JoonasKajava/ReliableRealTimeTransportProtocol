use crate::application_layer::connection_manager::SequenceNumber;
use log::info;

pub struct NewWindow {
    // TODO: Transmitter should have same kinda stuff, but for tracking time
    frame_status: Vec<bool>,
    window_size: u32,
    window_left_edge: u32,
}

impl Default for NewWindow {
    fn default() -> Self {
        Self {
            frame_status: vec![],
            window_size: u32::MAX / 2,
            window_left_edge: 0,
        }
    }
}

// There should be generic window that handles the frame status and the shifting
// Then there should be a transmitter and receiver that use the window

impl NewWindow {
    pub fn set_window_size(&mut self, size: u32) {
        self.window_size = size;
        self.frame_status.resize(size as usize, false);
    }

    pub fn get_window_size(&self) -> u32 {
        self.window_size
    }

    pub fn get_window_left_edge(&self) -> u32 {
        self.window_left_edge
    }

    pub fn get_window_index(&self, sequence_number: SequenceNumber) -> Option<usize> {
        match self.is_within_window(sequence_number) {
            false => None,
            true => Some((sequence_number - self.window_left_edge) as usize - 1),
        }
    }

    pub fn shift_window(&mut self) -> usize {
        let mut shift_amount = 0usize;
        for e in self.frame_status.iter() {
            if *e {
                shift_amount += 1;
            } else {
                break;
            }
        }
        self.frame_status.drain(0..shift_amount);
        self.window_left_edge += shift_amount as u32;
        shift_amount
    }

    pub fn is_within_window(&self, sequence_number: u32) -> bool {
        sequence_number > self.window_left_edge
            && sequence_number < self.window_left_edge + self.window_size
    }

    pub fn update_frame_status(&mut self, index: usize) {
        // TODO: There could be a better way to handle this
        if index >= self.frame_status.len() {
            let new_size = index + 1;
            info!("Resizing frame status to size {}", new_size);
            self.frame_status.resize(new_size, false);
        }
        self.frame_status[index] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_shift_when_frame_status_empty() {
        let mut window = NewWindow {
            frame_status: vec![],
            window_left_edge: 0,
            ..Default::default()
        };

        window.shift_window();

        assert_eq!(window.window_left_edge, 0);
    }
    #[test]
    fn no_shift_when_frame_status_full_of_false() {
        let mut window = NewWindow {
            frame_status: vec![false, false, false, false, false],
            window_left_edge: 0,
            ..Default::default()
        };

        window.shift_window();

        assert_eq!(window.window_left_edge, 0);
    }

    #[test]
    fn shift_when_possible() {
        let mut window = NewWindow {
            frame_status: vec![true, true, false, true, false],
            window_left_edge: 0,
            ..Default::default()
        };

        window.shift_window();

        assert_eq!(window.window_left_edge, 2);
        assert_eq!(window.frame_status, vec![false, true, false]);

        window.shift_window();

        assert_eq!(window.window_left_edge, 2);
        assert_eq!(window.frame_status, vec![false, true, false]);
    }
}
