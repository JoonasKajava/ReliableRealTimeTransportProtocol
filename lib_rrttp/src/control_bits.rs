use bitflags::bitflags;
bitflags! {
    #[repr(transparent)]
    pub struct ControlBits: u8 {
        /// Acknowledgment
        const ACK = 0b00000001;
        /// End of message
        const EOM = 0b00000010;
    }
}