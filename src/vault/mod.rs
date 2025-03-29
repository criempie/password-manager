use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PasswordEntry {
    pub id: String,
    pub login: String,
    password: String,
}

impl PasswordEntry {
    pub fn new(id: String, login: String, password: String) -> PasswordEntry {
        Self {
            id,
            login,
            password,
        }
    }
}

pub struct Vault_reborn {
    entries: Vec<PasswordEntry>,
}

impl Vault_reborn {
    pub fn new() -> Vault_reborn {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn generate_id() -> String {
        return Uuid::new_v4().as_simple().to_string();
    }

    pub fn add_entry(&mut self, entry: PasswordEntry) {
        self.entries.push(entry);
    }

    pub fn get_entry(&self, id: String) -> Option<PasswordEntry> {
        return self.entries.iter().find(|&entry| entry.id == id).cloned();
    }
}
