pub mod frame;
pub mod constants;
pub mod control_bits;
pub mod frame_status;
pub mod option;
pub mod receiver;
pub mod socket;
pub mod transmitter;
pub mod window;
mod connection_manager;
mod receiver_window;
mod transmitter_window;
mod window_manager;
mod queue_sender;


pub trait ExtractUDPData {
    fn consume_udp_data(self) -> Vec<u8>;
}