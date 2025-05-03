use std::{
  fs,
  io::{self, Read, Write},
  marker::PhantomData,
};

use serde::{de::DeserializeOwned, Serialize};

use super::{error, IDatabase};

pub struct DatabaseJSON<F: Serialize + DeserializeOwned> {
  file_path: String,

  __marker: PhantomData<F>,
}

impl<TFormat: Serialize + DeserializeOwned> IDatabase for DatabaseJSON<TFormat> {
  type DatabaseFormat = TFormat;

  fn new(file_path: String) -> Self {
    Self {
      file_path,

      __marker: PhantomData,
    }
  }

  fn init(&self) -> Result<(), error::Error> {
    if let Err(e) = self.check_db_availability() {
      return Err(e);
    }

    return Ok(());
  }

  fn check_db_availability(&self) -> Result<(), error::Error> {
    return match fs::OpenOptions::new().read(true).open(&self.file_path) {
      Err(e) => match e.kind() {
        io::ErrorKind::NotFound => Err(error::Error::DatabaseFileNotFound),
        _ => return Err(error::Error::Unhandled(e.to_string())),
      },
      Ok(_) => Ok(()),
    };
  }

  fn save(&mut self, data: &TFormat) -> Result<(), error::Error> {
    let data_to_write =
      serde_json::to_string_pretty(data).map_err(|_| error::Error::FailWhileWritingToFile)?;

    return DatabaseJSON::<TFormat>::write(&self.file_path, &data_to_write);
  }

  fn load(&self) -> Result<TFormat, error::Error> {
    let data = DatabaseJSON::<TFormat>::read(&self.file_path)?;
    let deserialized = serde_json::from_str::<TFormat>(&data)
      .map_err(|_| error::Error::Unhandled(String::from("Deserialization")))?;

    return Ok(deserialized);
  }
}

impl<F: Serialize + DeserializeOwned> DatabaseJSON<F> {
  fn write(file_path: &String, data: &String) -> Result<(), error::Error> {
    let file = fs::OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(file_path)
      .map_err(|_| error::Error::Unhandled(String::from("Fail on write 1")))?;

    let mut writer = io::BufWriter::new(file);

    writer
      .write_all(data.as_bytes())
      .map_err(|_| error::Error::Unhandled(String::from("Fail on write 2")))?;

    return Ok(());
  }

  fn read(file_path: &String) -> Result<String, error::Error> {
    let file = fs::OpenOptions::new()
      .read(true)
      .open(file_path)
      .map_err(|_| error::Error::Unhandled(String::from("Fail on read")))?;

    let mut buffer = String::new();
    let mut reader = io::BufReader::new(file);

    reader
      .read_to_string(&mut buffer)
      .map_err(|_| error::Error::Unhandled(String::from("Fail on read")))?;

    return Ok(buffer);
  }
}
