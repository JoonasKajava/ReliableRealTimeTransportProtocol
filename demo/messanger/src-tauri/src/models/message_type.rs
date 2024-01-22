use lib_rrttp::application_layer::message::MessageTypeTrait;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Message = 0,
    Acknowledgement = 1,
    FileInfo = 2,
    FileRejected = 3,
    FileAccepted = 4,
}

impl Into<u8> for MessageType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl MessageTypeTrait for MessageType {}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Message,
            1 => Self::Acknowledgement,
            2 => Self::FileInfo,
            3 => Self::FileRejected,
            4 => Self::FileAccepted,
            _ => panic!("Unknown request {}", value)
        }
    }
}