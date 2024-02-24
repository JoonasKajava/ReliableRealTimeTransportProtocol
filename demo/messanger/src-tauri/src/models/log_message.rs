use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::models::file_models::FileMetadata;

pub type LogMessageResult = Result<LogSuccessMessage, LogErrorMessage>;

#[typeshare]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "content")]
pub enum LogSuccessMessage {
    UnknownMessage(String),
    MessageReceived(String),
    MessageSent(String),
    LocalSocketBindSuccess(String),
    ConnectedToRemote(String),
    FileInfoSent(FileMetadata),
    FileInfoReceived(FileMetadata),
    FileDataReceived,
    ReceivedAcknowledgement,
    FileRejected(FileMetadata),
    FileAccepted(FileMetadata),
    FileResponseSent,
    Error(String),
}

#[typeshare]
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum LogErrorMessage {
    MessageSendError(String),
    LocalSocketBindFailed(String),
    LocalSocketNotBound,
    ConnectionError(String),
    FileSendError(String),
    InvalidFileResponse(String),
}
