use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct NetworkFileInfo {
    pub file_name: String,
    pub file_mime: Option<String>,
    pub file_size_in_bytes: u32,
}