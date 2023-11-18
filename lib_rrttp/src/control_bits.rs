use bitflags::bitflags;
bitflags! {
        #[repr(transparent)]
    pub struct ControlBits: u8 {
        const ACK = 0b00000001;
    }
}