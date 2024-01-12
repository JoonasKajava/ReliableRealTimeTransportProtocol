
use crate::transport_layer::ExtractUDPData;


pub trait MessageTypeTrait: From<u8> + Into<u8> + Sync + Send + Clone {}

#[derive(Debug, Clone)]
pub struct Message<TMessage: MessageTypeTrait> {
    pub message_type: TMessage,
    pub payload: Vec<u8>,
}

impl<TMessage: MessageTypeTrait> ExtractUDPData for Message<TMessage> {
    fn consume_udp_data(mut self) -> Vec<u8> {
        self.payload.insert(0, self.message_type.into());
        self.payload
    }
}


impl<TMessage: MessageTypeTrait> From<&[u8]> for Message<TMessage> {
    fn from(value: &[u8]) -> Self {
        let request_type = TMessage::from(value[0]);
        let payload = value[1..].to_vec();
        Self {
            message_type: request_type,
            payload,
        }
    }
}