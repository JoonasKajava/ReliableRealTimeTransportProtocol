use crate::models::network_file_info::NetworkFileInfo;

pub enum Message {
    String(String),
    FileInfo(NetworkFileInfo),
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
                let payload = NetworkFileInfo::try_from(payload)
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
    type Error = String;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let message_type: u32 = match &self {
            Self::String(_) => 0,
            Self::FileInfo(_) => 1,
            Self::ResponseToFileInfo { accepted: _ } => 2,
            Self::FileData(_) => 3,
        };

        let payload: Vec<u8> = match self {
            Self::String(payload) => payload.as_bytes().to_vec(),
            Self::FileInfo(payload) => payload.try_into()?,
            Self::ResponseToFileInfo { accepted } => {
                bincode::serialize(&accepted).map_err(|e| e.to_string())?
            }
            Self::FileData(payload) => payload.to_vec(),
        };
        Ok([message_type.to_be_bytes().to_vec(), payload].concat())
    }
}

pub enum MessageParsingError {
    InvalidMessageType(String),
    InvalidMessagePayload(String),
}
