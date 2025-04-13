use std::{
    fs,
    io::{self, Read, Write},
    marker::PhantomData,
};

use serde::{de::DeserializeOwned, Serialize};

use super::{format::IDatabaseFormat, IDatabase};

pub struct DatabaseJSON<F: Serialize + DeserializeOwned + IDatabaseFormat> {
    __marker: PhantomData<F>,
    file_path: String,
}

impl<F: Serialize + DeserializeOwned + IDatabaseFormat> IDatabase<F> for DatabaseJSON<F> {
    // Создание и заполнение пустыми данными файла, если он не существует.
    fn init(file_path: String) -> Result<Self, ()> {
        let file = fs::OpenOptions::new().open(&file_path);

        if let Err(e) = file {
            match e.kind() {
                // Если файла нет, то создаем его и заполняем пустыми данными.
                io::ErrorKind::NotFound => {
                    let _ = fs::OpenOptions::new().create(true).open(&file_path);

                    let clean_format = F::new();
                    let data_to_write = serde_json::to_string_pretty(&clean_format).unwrap();

                    DatabaseJSON::<F>::write(&file_path, &data_to_write).unwrap();
                }

                _ => {}
            }
        }

        return Ok(Self {
            __marker: PhantomData,
            file_path,
        });
    }

    fn save(&mut self, data: &F) -> Result<(), ()> {
        let data_to_write = serde_json::to_string_pretty(data).unwrap();

        return DatabaseJSON::<F>::write(&self.file_path, &data_to_write);
    }

    fn load(&self) -> Result<F, ()> {
        let data = DatabaseJSON::<F>::read(&self.file_path).unwrap();
        let deserialized = serde_json::from_str::<F>(&data).unwrap();

        return Ok(deserialized);
    }
}

impl<F: Serialize + DeserializeOwned + IDatabaseFormat> DatabaseJSON<F> {
    fn write(file_path: &String, data: &String) -> Result<(), ()> {
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap();

        let mut writer = io::BufWriter::new(file);
        writer.write_all(data.as_bytes()).unwrap();

        return Ok(());
    }

    fn read(file_path: &String) -> Result<String, ()> {
        let file = fs::OpenOptions::new().read(true).open(file_path).unwrap();
        let mut buffer = String::new();
        let mut reader = io::BufReader::new(file);

        reader.read_to_string(&mut buffer).unwrap();

        return Ok(buffer);
    }
}
