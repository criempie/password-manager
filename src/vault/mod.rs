pub mod database;
pub mod entry;

use database::Database;
use entry::Entry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultSettings {}

impl VaultSettings {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Vault {
    pub entries: Vec<Entry>,
    pub settings: VaultSettings,
    db: Database,
}

impl Vault {
    pub fn new() -> Vault {
        Self {
            entries: Vec::new(),
            settings: VaultSettings::new(),
            db: Database::new(),
        }
    }

    pub fn init(&mut self) {
        if let Err(e) = self.db.open() {
            panic!("db opening: {}", e);
        }

        let db_format = match self.db.unload() {
            Ok(data) => data,
            Err(e) => {
                panic!("db unloading: {}", e);
            }
        };

        self.entries = db_format.entries;
        self.settings = db_format.settings;
    }
}
