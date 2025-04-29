pub mod db_json;
pub mod error;
pub mod format;

use serde::{de::DeserializeOwned, Serialize};

const DB_PATH: &str = "./db.json";

pub trait IDatabase {
  type DatabaseFormat: Serialize + DeserializeOwned;

  fn new(file_path: String) -> Self;
  fn init(&self) -> Result<(), error::Error>;
  fn check_db_availability(&self) -> Result<(), error::Error>;
  fn save(&mut self, data: &Self::DatabaseFormat) -> Result<(), error::Error>;
  fn load(&self) -> Result<Self::DatabaseFormat, error::Error>;
}
