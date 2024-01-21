use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[typeshare]
pub struct NetworkFileInfo {
    pub file_name: String,
    pub file_mime: Option<String>,
    pub file_size_in_bytes: u32,
}