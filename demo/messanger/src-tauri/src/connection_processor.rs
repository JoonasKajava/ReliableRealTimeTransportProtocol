use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use log::{error, info};

use lib_rrttp::application_layer::connection_manager::ConnectionEventType;

use crate::message::Message;
use crate::models::log_message::LogSuccessMessage;
use crate::MessageState;

pub struct ConnectionProcessor {
    log_sender: Sender<LogSuccessMessage>,
    message_state: Arc<Mutex<MessageState>>,
}

impl ConnectionProcessor {
    pub fn new(
        log_sender: Sender<LogSuccessMessage>,
        message_state: Arc<Mutex<MessageState>>,
    ) -> Self {
        ConnectionProcessor {
            log_sender,
            message_state,
        }
    }
}

impl ConnectionProcessor {
    pub fn process_connection_event(&self, event: ConnectionEventType) -> Result<()> {
        info!("Processing connection event: {:?}", event);
        match event {
            ConnectionEventType::ReceivedFrame(_) => {
                // TODO: Inform client about file data received
            }
            ConnectionEventType::ReceivedCompleteMessage(message) => {
                self.process_message(message)?
            }
            ConnectionEventType::SentMessage => {}
            ConnectionEventType::SentFrame(_) => {
                // TODO: Inform client about file data sent
            }
            ConnectionEventType::ReceivedAck(_) => {}
            ConnectionEventType::SentAck(_) => {}
        }
        Ok(())
    }

    fn process_message(&self, message: Vec<u8>) -> Result<()> {
        let message = Message::try_from(message.as_slice())?;

        match message {
            Message::String(payload) => self
                .log_sender
                .send(LogSuccessMessage::MessageReceived(payload))?,
            Message::FileInfo(file_info) => {
                self.log_sender
                    .send(LogSuccessMessage::FileInfoReceived(file_info))?;
            }
            Message::ResponseToFileInfo { accepted } => {
                let log_message = match accepted {
                    true => LogSuccessMessage::FileAccepted,
                    false => LogSuccessMessage::FileRejected,
                };
                self.log_sender.send(log_message)?;
            }
            Message::FileData(_) => {
                self.log_sender.send(LogSuccessMessage::FileDataReceived)?;
            }
        };
        Ok(())
    }
}
