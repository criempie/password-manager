use serde::{Deserialize, Serialize};

use super::{
    database::format::IDatabaseFormat, entry::EntryEncrypted, vault_settings::VaultSettings,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseFormat {
    pub settings: VaultSettings,
    pub entries: Vec<EntryEncrypted>,
}

impl IDatabaseFormat for DatabaseFormat {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            settings: VaultSettings::default(),
        }
    }
}
