use serde::{self};
use std::{
    fs,
    io::{self, Read, Seek, Write},
    marker::{self},
    time,
};

const DB_FILE_PATH: &str = "./db.json";

pub struct Database<T: serde::Serialize + serde::de::DeserializeOwned> {
    _file: Option<fs::File>,
    __marker: marker::PhantomData<T>,
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> Database<T> {
    pub fn new() -> Database<T> {
        return Database {
            _file: None,
            __marker: marker::PhantomData {},
        };
    }
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> Database<T> {
    pub fn open(&mut self) -> Result<(), io::Error> {
        if let None = self._file {
            let file = fs::OpenOptions::new()
                .read(true)
                .create(true)
                .write(true)
                .open(DB_FILE_PATH)?;

            self._file = Some(file);

            return Ok(());
        }

        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "File already opened.",
        ));
    }

    pub fn close(&mut self) {
        if let Some(_) = self._file {
            self._file = None;
        }
    }

    pub fn load_from(&mut self) -> Result<T, io::Error> {
        match &mut self._file {
            Some(file) => {
                let mut buffer = String::new();

                file.seek(std::io::SeekFrom::Start(0))?;
                file.read_to_string(&mut buffer)?;

                let result = serde_json::from_str::<T>(&buffer)?;

                return Ok(result);
            }
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not opened.")),
        }
    }

    pub fn load_to(&mut self, data: &T) -> Result<(), io::Error> {
        match &mut self._file {
            Some(file) => {
                let serialized = serde_json::to_string_pretty(data)?;

                file.seek(io::SeekFrom::Start(0))?;
                file.set_len(0).unwrap();
                file.write_all(serialized.as_bytes())?;

                return Ok(());
            }
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not opened.")),
        }
    }

    pub fn password_next(&self) -> u128 {
        return time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
    }
}
