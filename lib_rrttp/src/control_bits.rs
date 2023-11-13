use bitflags::bitflags;
bitflags! {

    pub struct ControlBits: u8 {
        const ACK = 0b00000001;
    }
}