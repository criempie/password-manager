use std::{
    fs,
    io::{self, Read, Write},
    marker::PhantomData,
};

use serde::{de::DeserializeOwned, Serialize};

use super::{error, format::IDatabaseFormat, IDatabase};

pub struct DatabaseJSON<F: Serialize + DeserializeOwned + IDatabaseFormat> {
    __marker: PhantomData<F>,
    file_path: String,
}

impl<F: Serialize + DeserializeOwned + IDatabaseFormat> IDatabase<F> for DatabaseJSON<F> {
    // Создание и заполнение пустыми данными файла, если он не существует.
    fn init(file_path: String) -> Result<Self, error::Error> {
        let file = fs::OpenOptions::new().open(&file_path);

        if let Err(e) = file {
            match e.kind() {
                // Если файла нет, то создаем его и заполняем пустыми данными.
                io::ErrorKind::NotFound => {
                    let _ = fs::OpenOptions::new()
                        .create(true)
                        .open(&file_path)
                        .map_err(|_| error::Error::WhileCreatingFile)?;

                    let clean_format = F::new();
                    let data_to_write = serde_json::to_string_pretty(&clean_format)
                        .map_err(|_| error::Error::Serialization)?;

                    DatabaseJSON::<F>::write(&file_path, &data_to_write);
                }

                _ => return Err(error::Error::UnhandledIO(e.to_string())),
            }
        }

        return Ok(Self {
            __marker: PhantomData,
            file_path,
        });
    }

    fn save(&mut self, data: &F) -> Result<(), error::Error> {
        let data_to_write =
            serde_json::to_string_pretty(data).map_err(|_| error::Error::Serialization)?;

        return DatabaseJSON::<F>::write(&self.file_path, &data_to_write);
    }

    fn load(&self) -> Result<F, error::Error> {
        let data = DatabaseJSON::<F>::read(&self.file_path)?;
        let deserialized =
            serde_json::from_str::<F>(&data).map_err(|_| error::Error::Deserialization)?;

        return Ok(deserialized);
    }
}

impl<F: Serialize + DeserializeOwned + IDatabaseFormat> DatabaseJSON<F> {
    fn write(file_path: &String, data: &String) -> Result<(), error::Error> {
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .map_err(|_| error::Error::WhileOpeningFile(file_path.clone()))?;

        let mut writer = io::BufWriter::new(file);

        writer
            .write_all(data.as_bytes())
            .map_err(|_| error::Error::WhileWritingIntoFile)?;

        return Ok(());
    }

    fn read(file_path: &String) -> Result<String, error::Error> {
        let file = fs::OpenOptions::new()
            .read(true)
            .open(file_path)
            .map_err(|_| error::Error::WhileOpeningFile(file_path.clone()))?;

        let mut buffer = String::new();
        let mut reader = io::BufReader::new(file);

        reader
            .read_to_string(&mut buffer)
            .map_err(|_| error::Error::WhileReadingFile)?;

        return Ok(buffer);
    }
}
