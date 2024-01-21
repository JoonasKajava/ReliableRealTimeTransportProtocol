use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[typeshare]
pub struct NetworkFileInfo {
    pub file_name: String,
    pub file_mime: Option<String>,
    pub file_size_in_bytes: u32,
}


impl TryFrom<&[u8]> for NetworkFileInfo {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bincode::deserialize(value).map_err(|e| e.to_string())
    }
}

impl TryInto<Vec<u8>> for NetworkFileInfo {
    type Error = String;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serialize(&self).map_err(|e| e.to_string())
    }
}