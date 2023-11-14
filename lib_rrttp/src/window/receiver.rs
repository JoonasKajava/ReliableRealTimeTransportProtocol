use crate::window::Window;

pub struct Receiver {
    window: Window,
    highest_received: usize,
}