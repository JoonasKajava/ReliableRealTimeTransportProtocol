use crate::window::Window;

pub struct Transmitter {
    window: Window,
    highest_acknowledged: usize,
}