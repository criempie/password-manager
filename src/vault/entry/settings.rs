use serde::{Deserialize, Serialize};

use crate::vault::crypt::EncryptionIV;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntrySettings {
    pub iv: EncryptionIV,
}
