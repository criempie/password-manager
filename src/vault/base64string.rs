use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Base64String(pub String);

impl Base64String {
    pub fn encode(data: Vec<u8>) -> Self {
        Self(Base64String::_encode(&data))
    }

    pub fn from(str: String) -> Self {
        Self(str)
    }

    pub fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
        return general_purpose::STANDARD.decode(&self.0);
    }

    fn _encode(data: &[u8]) -> String {
        return general_purpose::STANDARD.encode(data);
    }
}
