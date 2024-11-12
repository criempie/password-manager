use std::{
    fs::{self},
    io::{Read, Seek, SeekFrom, Write},
};

use serde::{Deserialize, Serialize};

const DB_FILE_PATH: &str = "./db.json";

pub struct Vault {
    _file: fs::File,
}

#[derive(Serialize, Deserialize)]
pub struct VaultEntry {
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

        return Vault { _file: file };
    }

    pub fn entry_create(login: String, password: String) -> VaultEntry {
        return VaultEntry { login, password };
    }
}

impl Vault {
    pub fn read(&mut self) -> String {
        let mut content = String::new();
        self._file.seek(SeekFrom::Start(0)).unwrap();
        self._file.read_to_string(&mut content).unwrap();

        return content;
    }

    pub fn save(&mut self, entry: &VaultEntry) {}
}
