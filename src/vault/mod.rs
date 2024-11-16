use std::{
    collections::HashMap,
    fs::{self},
    io::{Read, Seek, SeekFrom, Write},
};

use serde::{Deserialize, Serialize};

pub mod database;

const DB_FILE_PATH: &str = "./db.json";

pub struct Vault {
    _file: fs::File,
    _entries: HashMap<u16, VaultEntry>,
    _increment: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VaultEntry {
    id: u16,
    login: String,
    password: String,
}

impl Vault {
    pub fn open() -> Vault {
        let file = fs::OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(DB_FILE_PATH)
            .unwrap();

        return Vault {
            _file: file,
            _entries: HashMap::new(),
            _increment: 1,
        };
    }
}

impl Vault {
    pub fn load_from_file(&mut self) {
        let mut content = String::new();

        self._file.seek(SeekFrom::Start(0)).unwrap();
        self._file.read_to_string(&mut content).unwrap();

        self._entries = serde_json::from_str(&content).unwrap();
    }

    pub fn save_to_file(&mut self) {
        let serialized = serde_json::to_string_pretty(&self._entries).unwrap();
        self._file.seek(SeekFrom::Start(0)).unwrap();
        self._file.write_all(&serialized.as_bytes()).unwrap();
    }

    pub fn entry_create(&mut self, login: String, password: String) -> VaultEntry {
        let id = self._increment;
        self._increment += 1;

        return VaultEntry {
            id,
            login,
            password,
        };
    }

    pub fn entry_save(&mut self, entry: VaultEntry) {
        self._entries.insert(entry.id, entry);
    }
}
