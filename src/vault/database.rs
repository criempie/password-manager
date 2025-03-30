use serde::{self, Deserialize, Serialize};
use std::{
    fs,
    io::{self, Read},
};
use uuid::Uuid;

use crate::{error::Error, vault::entry::Entry};

use super::VaultSettings;

const DB_PATH: &str = "./db.json";

#[derive(Serialize, Deserialize)]
pub struct DatabaseFormat {
    pub entries: Vec<Entry>,
    pub settings: VaultSettings,
}

pub struct Database {
    file: Option<fs::File>,
}

impl Database {
    pub fn new() -> Self {
        Self { file: None }
    }

    pub fn open(&mut self) -> Result<(), Error> {
        if let None = self.file {
            let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(DB_PATH)?;

            self.file = Some(file);

            return Ok(());
        }

        return Err(Error::DatabaseError("Already opened".to_string()));
    }

    pub fn close(&mut self) {
        self.file = None;
    }

    pub fn unload(&mut self) -> Result<DatabaseFormat, Error> {
        if let None = self.file {
            self.open()?;
        }

        let mut reader = io::BufReader::new(self.file.as_ref().unwrap());
        let mut buff = String::new();

        if let Err(e) = reader.read_to_string(&mut buff) {
            println!("{}", e);
        }

        let data = serde_json::from_str::<DatabaseFormat>(&buff)?;

        return Ok(data);
    }

    pub fn generate_id() -> String {
        return Uuid::new_v4().as_simple().to_string();
    }
}
