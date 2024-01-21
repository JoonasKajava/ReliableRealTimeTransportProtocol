use lib_rrttp::application_layer::connector::ConnectorEvents;

use crate::models::log_message::LogSuccessMessage;
use crate::models::message_type::MessageType;

impl From<ConnectorEvents<MessageType>> for LogSuccessMessage {
    fn from(event: ConnectorEvents<MessageType>) -> Self {
        match event {
            ConnectorEvents::MessageReceived(message) => {
                let network_message = String::from_utf8(message.payload).unwrap_or_else(|e| e.to_string());
                LogSuccessMessage::MessageReceived(network_message)
            }
        }
    }
}