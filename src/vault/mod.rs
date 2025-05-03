pub mod base64string;
pub mod database;
pub mod encryption_manager;
pub mod entry;
pub mod error;

use std::marker::PhantomData;

use base64string::Base64String;
use database::IDatabase;
use entry::IEntry;
use serde::{de::DeserializeOwned, ser::SerializeStruct, Deserialize, Serialize};
use uuid::Uuid;

use crate::random;

#[derive(Serialize, Deserialize)]
pub struct VaultDatabaseFormat<TEntry> {
  settings: VaultSettings,
  entries: Vec<TEntry>,
}

impl<TEntry> VaultDatabaseFormat<TEntry> {
  pub fn new(settings: VaultSettings, entries: Vec<TEntry>) -> Self {
    Self { settings, entries }
  }
}

#[derive(Debug)]
pub struct VaultSettings {
  master_password: String,
  encryption_key: Vec<u8>,
}

impl VaultSettings {
  pub fn new(master_password: String) -> Self {
    let encryption_key = random::generate_random_bytes(64);

    println!("{:?}", &encryption_key);

    Self {
      master_password,
      encryption_key,
    }
  }
}

impl Serialize for VaultSettings {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("VaultSettings", 2)?;

    state.serialize_field(
      "encryption_key",
      &Base64String::encode(&self.encryption_key).to_string(),
    )?;

    state.serialize_field("master_password", &self.master_password)?;

    return state.end();
  }
}

impl<'de> Deserialize<'de> for VaultSettings {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct Helper {
      master_password: String,
      encryption_key: Base64String,
    }

    let helper = Helper::deserialize(deserializer)?;

    let encryption_key = helper.encryption_key;
    let encryption_key = encryption_key.decode().unwrap();

    return Ok(VaultSettings {
      master_password: helper.master_password,
      encryption_key,
    });
  }
}

pub struct Vault<TDatabase, TEntry>
where
  TDatabase: IDatabase,
{
  database: TDatabase,

  pub settings: VaultSettings,
  entries: Vec<TEntry>,
}

impl<TDatabase, TEntry> Vault<TDatabase, TEntry>
where
  TDatabase: IDatabase<DatabaseFormat = VaultDatabaseFormat<TEntry>>,
{
  fn new(db: TDatabase, settings: VaultSettings, entries: Vec<TEntry>) -> Self {
    Self {
      database: db,
      entries,
      settings,
    }
  }

  pub fn initialize(db: TDatabase) -> Result<Self, error::Error> {
    if let Err(e) = db.init() {
      return Err(error::Error::DatabaseInitialization(e));
    }

    let data = match db.load() {
      Ok(data) => data,
      Err(e) => {
        return Err(error::Error::DatabaseLoad(e));
      }
    };

    return Ok(Self::new(db, data.settings, data.entries));
  }
}

impl<TDatabase, TEntry> Vault<TDatabase, TEntry>
where
  TDatabase: IDatabase,
  TEntry: IEntry,
{
  fn get_next_id(&self) -> String {
    return Uuid::new_v4().as_simple().to_string();
  }
}

// pub struct Vault<DB, DBFormat>
// where
//   DB: IDatabase<DBFormat>,
//   DBFormat: IDatabaseFormat,
// {
//   pub entries: Vec<EntryEncrypted>,
//   pub settings: VaultSettings,
//   db: DB,

//   __marker: PhantomData<DBFormat>,
// }

// impl<DB: IDatabase<DBFormat>, DBFormat: IDatabaseFormat> Vault<DB, DBFormat> {
//   pub fn new() -> Vault<DB, DBFormat> {
//     Self {
//       entries: Vec::new(),
//       settings: VaultSettings::new(),
//       db: DB::new(String::from("./db.json")),

//       __marker: PhantomData,
//     }
//   }

//   pub fn generate_id() -> String {
//     return Uuid::new_v4().as_simple().to_string();
//   }

//   pub fn init(&mut self) {
//     let mut data = match self.db.unload() {
//       Ok(data) => Ok(data),
//       Err(e) => match e {
//         DatabaseError::FileNotFound() => self.db.init_file(),
//         _ => panic!("{}", e.to_string()),
//       },
//     }
//     .unwrap();

//     if let None = data.settings.encryption_key {
//       data.settings.encryption_key = Some(EncryptionKey::generate_new());
//     }

//     self.entries = data.entries;
//     self.settings = data.settings;
//   }

//   pub fn create_entry(&mut self, login: String, password: String) -> Result<(), VaultError> {
//     let id = Vault::generate_id();

//     let entry = Entry::new(id, login, password);
//     let entry_encrypted = entry.encrypt(EncryptionKey::generate_new());

//     self.entries.push(entry_encrypted);

//     return Ok(());
//   }

//   pub fn get_entry(&self, id: &String) -> Result<Option<Entry>, VaultError> {
//     let entry = self.entries.iter().find(|&entry| entry.id == *id).cloned();

//     if let None = entry {
//       return Ok(None);
//     }

//     let entry = entry.unwrap();

//     match self.settings.encryption_key {
//       None => Err(VaultError::EncryptionKeyInvalid),
//       Some(key) => Ok(Some(entry.decrypt(key))),
//     }
//   }

//   pub fn delete_entry(&mut self, id: &String) -> Result<(), VaultError> {
//     let index = self.entries.iter().position(|entry| entry.id == *id);

//     match index {
//       None => return Err(VaultError::EntryNotFound),
//       Some(index) => self.entries.remove(index),
//     };
//     return Ok(());
//   }

//   pub fn save(&mut self) -> Result<(), DatabaseError> {
//     // let format = DatabaseFormat {
//     //     entries: self.entries.clone(),
//     //     settings: self.settings.clone(),
//     // };

//     // self.db.load(&format)?;

//     return Ok(());
//   }
// }
