use lib_rrttp::application_layer::connector::ConnectorEvents;

use crate::models::log_message::LogSuccessMessage;
use crate::models::message_type::MessageType;

impl From<ConnectorEvents<MessageType>> for LogSuccessMessage {
    fn from(event: ConnectorEvents<MessageType>) -> Self {
        match event {
            ConnectorEvents::MessageReceived(message) => {
                match message.message_type {
                    MessageType::Message => {
                        let network_message = String::from_utf8(message.payload).unwrap_or_else(|e| e.to_string());
                        LogSuccessMessage::MessageReceived(network_message)
                    }
                    MessageType::FileInfo => {
                        let network_file_info = match message.payload.as_slice().try_into() {
                            Ok(file_info) => file_info,
                            Err(e) => return LogSuccessMessage::UnknownMessage(e)
                        };
                        return LogSuccessMessage::FileInfoReceived(network_file_info);
                    }
                    MessageType::Acknowledgement => LogSuccessMessage::ReceivedAcknowledgement
                }
            }
        }
    }
}