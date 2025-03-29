use serde::{self, de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
use std::{
    fs,
    io::{self, Read},
};

use crate::{error::Error, vault::PasswordEntry};

const DB_PATH: &str = "./db.json";

#[derive(Serialize, Deserialize)]
struct DatabaseFormat {
    entries: Vec<PasswordEntry>,
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

    pub fn get_entries(&mut self) -> Result<Vec<PasswordEntry>, Error> {
        if let None = self.file {
            self.open()?;
        }

        let mut reader = io::BufReader::new(self.file.as_ref().unwrap());
        let mut buff = String::new();

        if let Err(e) = reader.read_to_string(&mut buff) {
            println!("{}", e);
        }

        let data: DatabaseFormat = serde_json::from_str(&buff)?;

        return Ok(data.entries);
    }
}
