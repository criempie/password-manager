pub mod base64string;
pub mod database;
pub mod entry;

mod crypt;

use crate::error::{DatabaseError, VaultError};
use crypt::EncryptionKey;
use database::Database;
use entry::{Entry, EntryEncrypted};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultSettings {
    encryption_key: Option<EncryptionKey>,
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

pub struct Vault {
    pub entries: Vec<EntryEncrypted>,
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
        let mut data = match self.db.unload() {
            Ok(data) => Ok(data),
            Err(e) => match e {
                DatabaseError::FileNotFound() => self.db.init_file(),
                _ => panic!("{}", e.to_string()),
            },
        }
        .unwrap();

        if let None = data.settings.encryption_key {
            data.settings.encryption_key = Some(EncryptionKey::generate_new());
        }

        self.entries = data.entries;
        self.settings = data.settings;
    }

    pub fn create_entry(&mut self, login: String, password: String) -> Result<(), VaultError> {
        let id = Vault::generate_id();

        let entry = Entry::new(id, login, password);
        let entry_encrypted = entry.encrypt(EncryptionKey::generate_new());

        self.entries.push(entry_encrypted);

        return Ok(());
    }

    pub fn get_entry(&self, id: &String) -> Result<Option<Entry>, VaultError> {
        let entry = self.entries.iter().find(|&entry| entry.id == *id).cloned();

        if let None = entry {
            return Ok(None);
        }

        let entry = entry.unwrap();

        match self.settings.encryption_key {
            None => Err(VaultError::EncryptionKeyInvalid),
            Some(key) => Ok(Some(entry.decrypt(key))),
        }
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
        // let format = DatabaseFormat {
        //     entries: self.entries.clone(),
        //     settings: self.settings.clone(),
        // };

        // self.db.load(&format)?;

        return Ok(());
    }
}
