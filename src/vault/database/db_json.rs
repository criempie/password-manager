use std::{
  fs,
  io::{self, Read, Write},
  marker::PhantomData,
};

use serde::{de::DeserializeOwned, Serialize};

use super::{error, format::IDatabaseFormat, IDatabase};

pub struct DatabaseJSON<F: IDatabaseFormat> {
  __marker: PhantomData<F>,
  file_path: String,
}

impl<TFormat: IDatabaseFormat> IDatabase for DatabaseJSON<TFormat> {
  type DatabaseFormat = TFormat;

  fn new(file_path: String) -> Self {
    Self {
      __marker: PhantomData,
      file_path,
    }
  }

  /*
    Result<(), NotFound | UnhandledIO>
  */
  fn check_db_availability(&self) -> Result<(), error::Error> {
    println!("{}", &self.file_path);
    return match fs::OpenOptions::new().read(true).open(&self.file_path) {
      Err(e) => match e.kind() {
        io::ErrorKind::NotFound => Err(error::Error::DatabaseFileNotFound),
        _ => return Err(error::Error::Unhandled(e.to_string())),
      },
      Ok(_) => Ok(()),
    };
  }

  fn init(&self) -> Result<(), error::Error> {
    return Ok(());
  }

  fn save(&mut self, data: &TFormat) -> Result<(), error::Error> {
    let data_to_write = serde_json::to_string_pretty(data)
      .map_err(|_| error::Error::Unhandled(String::from("Serilization")))?;

    return DatabaseJSON::<TFormat>::write(&self.file_path, &data_to_write);
  }

  fn load(&self) -> Result<TFormat, error::Error> {
    let data = DatabaseJSON::<TFormat>::read(&self.file_path)?;
    let deserialized = serde_json::from_str::<TFormat>(&data)
      .map_err(|_| error::Error::Unhandled(String::from("Deserialization")))?;

    return Ok(deserialized);
  }
}

impl<F: Serialize + DeserializeOwned + IDatabaseFormat> DatabaseJSON<F> {
  fn write(file_path: &String, data: &String) -> Result<(), error::Error> {
    let file = fs::OpenOptions::new()
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
