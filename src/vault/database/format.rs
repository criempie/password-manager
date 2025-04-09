use serde::{Deserialize, Serialize};

use crate::vault::{entry::EntryEncrypted, VaultSettings};

#[derive(Serialize, Deserialize)]
pub struct DatabaseFormat {
    pub entries: Vec<EntryEncrypted>,
    pub settings: VaultSettings,
}

impl DatabaseFormat {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            settings: VaultSettings::new(),
        }
    }
}
