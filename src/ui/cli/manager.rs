use serde::{Deserialize, Serialize};

use crate::vault::{
  database::{db_json::DatabaseJSON, format::IDatabaseFormat, IDatabase},
  entry::Entry,
  IVault, Vault,
};

pub struct Manager {}

#[derive(Serialize, Deserialize)]
struct DefaultCredentials {
  login: String,
  password: String,
}

#[derive(Serialize, Deserialize)]
struct DatabaseDefaultFormat {
  entries: Vec<Entry<DefaultCredentials>>,
}

impl IDatabaseFormat for DatabaseDefaultFormat {
  fn new() -> Self {
    Self {
      entries: Vec::new(),
    }
  }
}

impl Manager {
  pub fn start() {
    /*
       TODO: Создание хранилища (vault)
       Попытка загрузки файла базы данных.
       Если файла не существует,
       сообщить об этом и предложить инициализировать хранилище (создать все ключи и сохранить в формат).
    */

    let vault = Vault::<DatabaseJSON<DatabaseDefaultFormat>, Entry<DefaultCredentials>>::new();

    println!("{:?}", vault.database.check_db_availability());

    // loop {
    //   println!();
    // }
  }
}
