use crate::application_layer::message_type::MessageType;
use crate::transport_layer::ExtractUDPData;

#[derive(Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}

impl ExtractUDPData for Message {
    fn consume_udp_data(mut self) -> Vec<u8> {
        self.payload.insert(0, self.message_type as u8);
        self.payload
    }
}


impl From<&[u8]> for Message {
    fn from(value: &[u8]) -> Self {
        let request_type = MessageType::from(value[0]);
        let payload = value[1..].to_vec();
        Self {
            message_type: request_type,
            payload,
        }
    }
}