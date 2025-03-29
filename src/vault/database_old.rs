use serde::{self, de::DeserializeOwned, Serialize};
use std::{
    fs,
    io::{self, Read, Seek, Write},
    marker::{self},
};

const DB_FILE_PATH: &str = "./db.json";

pub struct Database<T: Serialize + DeserializeOwned> {
    file: Option<fs::File>,
    entries: Vec<T>,
    __marker: marker::PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> Database<T> {
    pub fn new() -> Database<T> {
        return Database {
            file: None,
            entries: Vec::new(),
            __marker: marker::PhantomData {},
        };
    }

    pub fn open(&mut self) -> Result<(), io::Error> {
        if let None = self.file {
            let file = fs::OpenOptions::new()
                .read(true)
                .create(true)
                .write(true)
                .open(DB_FILE_PATH)?;

            self.file = Some(file);

            return Ok(());
        }

        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "File already opened.",
        ));
    }

    pub fn close(&mut self) {
        if let Some(_) = self.file {
            self.file = None;
        }
    }

    pub fn entries_get(&self) -> &Vec<T> {
        return &self.entries;
    }

    pub fn entry_get(&self, id: &str) -> Option<&T> {
        todo!();
    }

    pub fn entry_insert(&mut self, entry: T) {
        self.entries.push(entry);
    }

    pub fn entry_delete(&mut self, id: &str) {
        todo!();
    }

    pub fn sync_from_file(&mut self) -> Result<(), io::Error> {
        match &mut self.file {
            Some(file) => {
                let mut buffer = String::new();

                file.seek(std::io::SeekFrom::Start(0))?;
                file.read_to_string(&mut buffer)?;

                self.entries = serde_json::from_str::<Vec<T>>(&buffer)?;

                return Ok(());
            }
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not opened.")),
        }
    }

    pub fn sync_to_file(&mut self) -> Result<(), io::Error> {
        return match &mut self.file {
            Some(file) => {
                let serialized = serde_json::to_string_pretty(&self.entries)?;

                file.seek(io::SeekFrom::Start(0))?;
                file.set_len(0).unwrap();
                file.write_all(serialized.as_bytes())?;

                return Ok(());
            }
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not opened.")),
        };
    }
}
