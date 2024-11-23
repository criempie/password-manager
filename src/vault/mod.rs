use serde::{Deserialize, Serialize};
use std::io::{self};

mod database;
use database::Database;

#[derive(Serialize, Deserialize, Debug)]
pub struct VaultEntry {
    id: u128,
    login: String,
    password: String,
}

pub struct Vault {
    db: Database<Vec<VaultEntry>>,
    entries: Vec<VaultEntry>,
}

impl Vault {
    pub fn new() -> Vault {
        return Vault {
            db: Database::new(),
            entries: Vec::new(),
        };
    }

    pub fn entry_create(&self, login: &str, password: &str) -> VaultEntry {
        let entry = VaultEntry {
            id: self.db.password_next(),
            login: String::from(login),
            password: String::from(password),
        };

        return entry;
    }

    pub fn entry_save(&mut self, entry: VaultEntry) {
        self.entries.push(entry);
    }

    pub fn entries_get(&self) -> &Vec<VaultEntry> {
        return &self.entries;
    }

    pub fn load_from_db(&mut self) -> Result<(), io::Error> {
        match self.db.load_from() {
            Ok(entries) => self.entries = entries,
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => {
                    self.db.open()?;
                    self.load_from_db()?;
                }
                _ => {
                    return Err(error);
                }
            },
        }

        return Ok(());
    }

    pub fn save_to_db(&mut self) -> Result<(), io::Error> {
        self.db.load_to(&self.entries)?;

        return Ok(());
    }
}
