use crate::message::Message;
use crate::models::log_message::LogSuccessMessage;
use lib_rrttp::application_layer::connection_manager::ConnectionEventType;
use log::{error, info};
use std::sync::mpsc::Sender;

pub struct ConnectionProcessor {
    log_sender: Sender<LogSuccessMessage>,
}

impl ConnectionProcessor {
    pub fn new(log_sender: Sender<LogSuccessMessage>) -> Self {
        ConnectionProcessor { log_sender }
    }
}

impl ConnectionProcessor {
    pub fn process_connection_event(&self, event: ConnectionEventType) {
        info!("Processing connection event: {:?}", event);
        match event {
            ConnectionEventType::ReceivedFrame(_) => {}
            ConnectionEventType::ReceivedCompleteMessage(message) => self.process_message(message),
            ConnectionEventType::SentMessage => {}
            ConnectionEventType::SentFrame(_) => {}
            ConnectionEventType::ReceivedAck(_) => {}
            ConnectionEventType::SentAck(_) => {}
        }
    }

    fn process_message(&self, message: Vec<u8>) {
        let message = Message::try_from(message.as_slice());
        match message {
            Ok(message) => {
                match message {
                    Message::String(payload) => {
                        if let Err(e) = self
                            .log_sender
                            .send(LogSuccessMessage::MessageReceived(payload))
                        {
                            error!("Error sending log message: {}", e);
                        }
                    }
                    Message::FileInfo(_) => {}
                    Message::ResponseToFileInfo { .. } => {}
                    Message::FileData(_) => {}
                };
            }
            Err(_) => {}
        }
    }
}
