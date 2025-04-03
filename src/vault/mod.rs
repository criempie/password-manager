pub mod database;
pub mod entry;

mod crypt;

use crate::error::{CryptError, DatabaseError, VaultError};
use crypt::{base64_to_bytes, bytes_to_base64, EncryptionPairFold};
use database::{Database, DatabaseFormat};
use entry::{Entry, EntryEncrypted};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultSettings {
    encryption_key: Option<String>,
}

impl VaultSettings {
    pub fn new() -> Self {
        Self {
            encryption_key: None,
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

    pub fn create_entry(&mut self, login: String, password: String) -> Result<(), VaultError> {
        let id = Vault::generate_id();

        let pair = crypt::EncryptionPair::generate();

        let entry = Entry::new(id, login, password);
        let entry_encrypted = entry.encrypt(pair);

        self.entries.push(entry_encrypted);

        return Ok(());
    }

    pub fn get_entry(&self, id: &String) -> Result<Option<Entry>, VaultError> {
        let entry = self.entries.iter().find(|&entry| entry.id == *id).cloned();

        if let None = entry {
            return Ok(None);
        }

        let entry = entry.unwrap();

        let encryption_pair = match self.settings.encryption_key.clone() {
            None => Err(VaultError::EncryptionKeyInvalid),
            Some(key) => {
                EncryptionPairFold::unfold(key).map_err(|_| VaultError::EncryptionKeyInvalid)
            }
        }?;

        return Ok(Some(entry.decrypt(encryption_pair)));
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
