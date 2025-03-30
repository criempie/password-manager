use serde::{self, Deserialize, Serialize};
use std::{
    fs,
    io::{self, Read, Write},
};

use crate::{error::DatabaseError, vault::entry::Entry};

use super::VaultSettings;

const DB_PATH: &str = "./db.json";

#[derive(Serialize, Deserialize)]
pub struct DatabaseFormat {
    pub entries: Vec<Entry>,
    pub settings: VaultSettings,
}

impl DatabaseFormat {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            settings: VaultSettings {},
        }
    }
}

pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Self {}
    }

    // Выгрузить из файла
    pub fn unload(&self) -> Result<DatabaseFormat, DatabaseError> {
        let file = match fs::OpenOptions::new().read(true).open(DB_PATH) {
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Err(DatabaseError::FileNotFound()),
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(file) => Ok(file),
        }?;

        let mut reader = io::BufReader::new(file);
        let mut buff = String::new();

        match reader.read_to_string(&mut buff) {
            Err(e) => match e.kind() {
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(_) => Ok(()),
        }?;

        let data = match serde_json::from_str::<DatabaseFormat>(&buff) {
            Err(e) => match e.classify() {
                serde_json::error::Category::Syntax | serde_json::error::Category::Data => {
                    Err(DatabaseError::FileFormatInvalid())
                }
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(data) => Ok(data),
        }?;

        return Ok(data);
    }

    // Загрузить в файл
    pub fn load(&self, data: &DatabaseFormat) -> Result<(), DatabaseError> {
        let file = match fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(DB_PATH)
        {
            Err(e) => match e.kind() {
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(file) => Ok(file),
        }?;

        let data_string = match serde_json::to_string_pretty(data) {
            Err(e) => match e.classify() {
                serde_json::error::Category::Syntax | serde_json::error::Category::Data => {
                    Err(DatabaseError::FileFormatInvalid())
                }
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(string) => Ok(string),
        }?;

        let mut writer = io::BufWriter::new(file);
        match writer.write_all(data_string.as_bytes()) {
            Err(e) => match e.kind() {
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(()) => Ok(()),
        }?;

        return Ok(());
    }

    pub fn init_file(&self) -> Result<DatabaseFormat, DatabaseError> {
        let file = match fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(DB_PATH)
        {
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists => Err(DatabaseError::FileAlreadyExist()),
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(file) => Ok(file),
        }?;

        let data = DatabaseFormat::empty();

        let data_string = match serde_json::to_string_pretty(&data) {
            Err(e) => match e.classify() {
                serde_json::error::Category::Syntax | serde_json::error::Category::Data => {
                    Err(DatabaseError::FileFormatInvalid())
                }
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(string) => Ok(string),
        }?;

        let mut writer = io::BufWriter::new(file);
        match writer.write_all(data_string.as_bytes()) {
            Err(e) => match e.kind() {
                _ => Err(DatabaseError::Any(e.to_string())),
            },
            Ok(()) => Ok(()),
        }?;

        return Ok(data);
    }
}
