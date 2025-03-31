pub mod database;
pub mod entry;

use crate::error::{DatabaseError, VaultError};
use database::{Database, DatabaseFormat};
use entry::Entry;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    pub fn generate_id() -> String {
        return Uuid::new_v4().as_simple().to_string();
    }

    pub fn init(&mut self) {
        let data = match self.db.unload() {
            Ok(data) => Ok(data),
            Err(e) => match e {
                DatabaseError::FileNotFound() => self.db.init_file(),
                _ => panic!("{}", e.to_string()),
            },
        }
        .unwrap();

        self.entries = data.entries;
        self.settings = data.settings;
    }

    pub fn create_entry(&mut self, login: String, password: String) {
        let id = Vault::generate_id();

        self.entries.push(Entry::new(id, login, password));
    }

    pub fn get_entry(&self, id: &String) -> Option<Entry> {
        return self.entries.iter().find(|&entry| entry.id == *id).cloned();
    }

    pub fn delete_entry(&mut self, id: &String) -> Result<(), VaultError> {
        let index = self.entries.iter().position(|entry| entry.id == *id);

        match index {
            None => return Err(VaultError::EntryNotFound),
            Some(index) => self.entries.remove(index),
        };

        return Ok(());
    }

    pub fn save(&mut self) -> Result<(), DatabaseError> {
        let format = DatabaseFormat {
            entries: self.entries.clone(),
            settings: self.settings.clone(),
        };

        self.db.load(&format)?;

        return Ok(());
    }
}
