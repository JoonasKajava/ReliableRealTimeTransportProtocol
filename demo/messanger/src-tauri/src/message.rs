use thiserror::Error;

use crate::models::file_models::FileMetadata;

pub enum Message {
    String(String),
    FileInfo(FileMetadata),
    ResponseToFileInfo { accepted: bool },
    FileData(Vec<u8>),
}

impl TryFrom<&[u8]> for Message {
    type Error = MessageParsingError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let message_type = value.get(0).ok_or_else(|| {
            MessageParsingError::InvalidMessageType("Unable to read message type".to_string())
        })?;

        let payload = value.get(1..).ok_or_else(|| {
            MessageParsingError::InvalidMessagePayload("Unable to read message payload".to_string())
        })?;
        match message_type {
            0 => {
                let payload = String::from_utf8(payload.to_vec())
                    .map_err(|e| MessageParsingError::InvalidMessagePayload(e.to_string()))?;
                Ok(Self::String(payload))
            }
            1 => {
                let payload = FileMetadata::try_from(payload)
                    .map_err(|e| MessageParsingError::InvalidMessagePayload(e))?;
                Ok(Self::FileInfo(payload))
            }
            2 => {
                let payload = bincode::deserialize(payload)
                    .map_err(|e| MessageParsingError::InvalidMessagePayload(e.to_string()))?;
                Ok(Self::ResponseToFileInfo { accepted: payload })
            }
            3 => Ok(Self::FileData(payload.to_vec())),
            _ => Err(MessageParsingError::InvalidMessageType(
                "Unknown message type".to_string(),
            )),
        }
    }
}

impl TryInto<Vec<u8>> for Message {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let message_type: u8 = match &self {
            Self::String(_) => 0,
            Self::FileInfo(_) => 1,
            Self::ResponseToFileInfo { accepted: _ } => 2,
            Self::FileData(_) => 3,
        };

        let payload: Vec<u8> = match self {
            Self::String(payload) => payload.as_bytes().to_vec(),
            Self::FileInfo(payload) => payload.try_into()?,
            Self::ResponseToFileInfo { accepted } => bincode::serialize(&accepted)?,
            Self::FileData(payload) => payload.to_vec(),
        };
        Ok([message_type.to_be_bytes().to_vec(), payload].concat())
    }
}

#[derive(Error, Debug)]
pub enum MessageParsingError {
    #[error("Invalid message type: {0}")]
    InvalidMessageType(String),
    #[error("Invalid message payload: {0}")]
    InvalidMessagePayload(String),
}
