pub mod frame;
pub mod constants;
pub mod control_bits;
pub mod frame_status;
pub mod option;
pub mod socket;
pub mod window;
pub(crate) mod receiver_window;
pub(crate) mod transmitter_window;


pub trait ExtractUDPData {
    fn consume_udp_data(self) -> Vec<u8>;
}