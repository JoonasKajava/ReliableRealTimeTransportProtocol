use crate::socket::Socket;
use crate::window::Window;

pub struct Receiver {
    window: Window,
    socket: Socket,
    highest_received: usize,
}

impl Receiver {
    pub fn new(addr: &str) -> std::io::Result<Self> {
        Ok(Self {
            window: Window::default(),
            highest_received: 0,
            socket: Socket::bind(addr)?,
        })
    }
    pub fn read(&mut self) {

    }
}