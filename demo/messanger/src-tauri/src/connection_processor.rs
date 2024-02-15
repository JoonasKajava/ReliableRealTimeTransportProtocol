use std::fs;
use std::sync::mpsc::{Sender, SyncSender};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use log::{error, info};
use thiserror::Error;

use lib_rrttp::application_layer::connection_manager::ConnectionEventType;

use crate::message::Message;
use crate::models::file_models::FileInfo;
use crate::models::log_message::LogSuccessMessage;
use crate::MessageState;

pub struct ConnectionProcessor {
    log_sender: Sender<LogSuccessMessage>,
    socket_sender: SyncSender<Vec<u8>>,
    message_state: Arc<Mutex<MessageState>>,
}

impl ConnectionProcessor {
    pub fn new(
        log_sender: Sender<LogSuccessMessage>,
        message_state: Arc<Mutex<MessageState>>,
        socket_sender: SyncSender<Vec<u8>>,
    ) -> Self {
        ConnectionProcessor {
            log_sender,
            message_state,
            socket_sender,
        }
    }
}

impl ConnectionProcessor {
    pub fn process_connection_event(&self, event: ConnectionEventType) -> Result<()> {
        info!("Processing connection event: {:?}", event);
        match event {
            ConnectionEventType::ReceivedFrame(_) => {}
            ConnectionEventType::ReceivedCompleteMessage(message) => {
                self.process_message(message)?
            }
            ConnectionEventType::SentMessage => {}
            ConnectionEventType::SentFrame(_) => {}
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
                self.message_state.lock().unwrap().incoming_file = Some(FileInfo {
                    metadata: file_info.clone(),
                    src: None,
                });
                self.log_sender
                    .send(LogSuccessMessage::FileInfoReceived(file_info))?;
            }
            Message::ResponseToFileInfo { accepted } => {
                let outgoing_file = self
                    .message_state
                    .lock()
                    .unwrap()
                    .outgoing_file
                    .clone()
                    .ok_or_else(|| {
                        error!("No outgoing file to respond to");
                        ConnectionProcessorErrors::GotResponseToUnknownFile
                    })?;
                let log_message = match accepted {
                    true => LogSuccessMessage::FileAccepted(outgoing_file.metadata.clone()),
                    false => LogSuccessMessage::FileRejected(outgoing_file.metadata.clone()),
                };
                let file_src = outgoing_file.src.clone().ok_or_else(|| {
                    error!("No file source to respond to");
                    ConnectionProcessorErrors::NoFileToSend
                })?;

                let file_data = fs::read(file_src)?;
                let file_message = Message::FileData(file_data);
                let payload = file_message.try_into()?;
                self.socket_sender.send(payload)?;
                self.log_sender.send(log_message)?;
            }
            Message::FileData(file_data) => {
                let incoming_file = self
                    .message_state
                    .lock()
                    .unwrap()
                    .incoming_file
                    .clone()
                    .ok_or_else(|| {
                        error!("State does not contain incoming file info");
                        ConnectionProcessorErrors::UnableToSaveFileData(
                            "State does not contain incoming file info".to_string(),
                        )
                    })?;
                let file_path = incoming_file.src.clone().ok_or_else(|| {
                    error!("Incoming file info does not contain file path");
                    ConnectionProcessorErrors::UnableToSaveFileData(
                        "Incoming file info does not contain file path".to_string(),
                    )
                })?;
                fs::write(file_path, file_data.as_slice())?;
                self.log_sender.send(LogSuccessMessage::FileDataReceived)?;
            }
        };
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ConnectionProcessorErrors {
    #[error("Remote responded to unknown file metadata")]
    GotResponseToUnknownFile,
    #[error("No src found when trying to send file data")]
    NoFileToSend,
    #[error("Unable to save file data")]
    UnableToSaveFileData(String),
}
