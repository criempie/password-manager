pub mod db_json;
pub mod error;
pub mod format;

use serde::{de::DeserializeOwned, Serialize};

const DB_PATH: &str = "./db.json";

pub trait IDatabase<F: Serialize + DeserializeOwned> {
    fn save(&mut self, data: &F) -> Result<(), error::Error>;
    fn load(&self) -> Result<F, error::Error>;
    fn init(file_path: String) -> Result<Self, error::Error>
    where
        Self: Sized;
}
