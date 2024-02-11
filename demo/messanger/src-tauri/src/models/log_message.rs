use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::models::network_file_info::NetworkFileInfo;

pub type LogMessageResult = Result<LogSuccessMessage, LogErrorMessage>;

#[typeshare]
#[derive(Clone, Serialize, Deserialize,Debug)]
#[serde(tag = "type", content = "content")]
pub enum LogSuccessMessage {
    UnknownMessage(String),
    MessageReceived(String),
    MessageSent(String),
    LocalSocketBindSuccess(String),
    ConnectedToRemote(String),
    FileInfoSent(NetworkFileInfo),
    FileInfoReceived(NetworkFileInfo),
    FileDataReceived,
    ReceivedAcknowledgement,
    FileRejected,
    FileAccepted,
    FileResponseSent,
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
