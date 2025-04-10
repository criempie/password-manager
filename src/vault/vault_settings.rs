use serde::{Deserialize, Serialize};

use super::crypt::EncryptionKey;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultSettings {
    pub encryption_key: Option<EncryptionKey>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultSettingsForLoad {}

impl VaultSettings {
    pub fn new() -> Self {
        Self {
            encryption_key: Some(EncryptionKey::generate_new()),
        }
    }
}
