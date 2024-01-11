
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    SendMessage = 0,
    SendFile = 1,
    ReceivedMessage = 2,
    ReadyToReceiveMessage = 3
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::SendMessage,
            1 => Self::SendFile,
            2 => Self::ReceivedMessage,
            3 => Self::ReadyToReceiveMessage,
            _ => panic!("Unknown request {}", value)
        }
    }
}