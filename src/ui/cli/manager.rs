use std::io::{stdin, stdout, Read, Write};

use serde::{Deserialize, Serialize};

use crate::vault::{
  self,
  database::{self, db_json::DatabaseJSON, format::IDatabaseFormat, IDatabase},
  entry::Entry,
  Vault, VaultDatabaseFormat, VaultSettings,
};

pub struct Manager {}

#[derive(Serialize, Deserialize)]
struct DefaultCredentials {
  login: String,
  password: String,
}

type DB = DatabaseJSON<VaultDatabaseFormat<Entry<DefaultCredentials>>>;
type MyVault = Vault<DB, Entry<DefaultCredentials>>;

impl Manager {
  pub fn start() {
    /*
      Создание базы данных, попытка инициализации.
      Если файла базы не существует, предлагается инициализировать новое хранилище.
      Инициализация хранилища - это создание ввод нового мастер пароля, хеширование мастер пароля (unlockKey),
      генерация ключа шифрования данных (encKey), генерация пары публичного и приватного ключей (pubKey, privKey).

      unlockKey и encKey существуют раздельно и связаны с помощью пары pubKey, privKey.
    */

    let mut db = DB::new(String::from("./db.json"));

    if let Err(e) = db.init() {
      if let database::error::Error::DatabaseFileNotFound = e {
        let settings = ifDatabaseNotFound_prompt();

        if let None = settings {
          return;
        }

        let format = VaultDatabaseFormat::new(settings.unwrap(), Vec::new());

        if let Err(e) = db.save(&format) {
          if let database::error::Error::FailWhileWritingToFile = e {
            eprintln!("Error during vault initiazation: {}.", e.to_string());
            return;
          }
        }
      } else {
        eprintln!("Error during database initialization: {}.", e.to_string());
        return;
      }
    }

    let mut vault = MyVault::initialize(db).unwrap();

    println!("{:?}", &vault.settings);

    // loop {
    //   println!();
    // }
  }
}

#[allow(non_snake_case)]
fn ifDatabaseNotFound_prompt() -> Option<VaultSettings> {
  print!("Database file not found. Initialize new? ");
  let is_want_to_initialize = yesno_prompt();

  if is_want_to_initialize {
    return Some(ifWantToInitialize_prompt());
  }

  return None;
}

#[allow(non_snake_case)]
fn ifWantToInitialize_prompt() -> VaultSettings {
  println!();
  print!("Enter master password: ");
  stdout().flush().unwrap();

  let mut buff = String::new();
  stdin().read_line(&mut buff).unwrap();

  move_cursor_up(1);
  move_cursor_to_start();
  print!(
    "Enter master password: {}\n",
    "*".repeat(buff.chars().count()),
  );
  stdout().flush().unwrap();

  let master_password = String::from(buff.trim());

  return VaultSettings::new(master_password);
}

fn yesno_prompt() -> bool {
  print!("(y/N): ");
  stdout().flush().unwrap();

  let mut buff = String::new();
  stdin().read_line(&mut buff).unwrap();

  return buff.trim().to_lowercase() == String::from("y");
}

fn move_cursor_up(n: u8) {
  print!("\x1B[{}A", n);
  stdout().flush().unwrap();
}

fn move_cursor_to_start() {
  print!("\r");
  stdout().flush().unwrap();
}
