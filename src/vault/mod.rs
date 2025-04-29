pub mod base64string;
pub mod database;
pub mod encryption_manager;
pub mod entry;
mod error;

use std::marker::PhantomData;

use database::IDatabase;
use entry::IEntry;
use uuid::Uuid;

pub struct Vault<TDatabase, TEntry>
where
  TDatabase: IDatabase,
  TEntry: IEntry,
{
  pub database: TDatabase,

  __marker_1: PhantomData<TEntry>,
}

pub trait IVault {
  type VaultDatabase: IDatabase;
  type VaultEntry: IEntry;

  fn new() -> Self;
  fn init(&mut self) -> Result<(), error::Error>;
  fn create_entry(
    &mut self,
    credentials: <Self::VaultEntry as IEntry>::Credentials,
  ) -> Result<Self::VaultEntry, error::Error>;
  fn get_entry(&mut self) -> Result<Self::VaultEntry, error::Error>;
}

impl<TDatabase, TEntry> IVault for Vault<TDatabase, TEntry>
where
  TDatabase: IDatabase,
  TEntry: IEntry,
{
  type VaultDatabase = TDatabase;
  type VaultEntry = TEntry;

  fn new() -> Self {
    Self {
      database: TDatabase::new(String::from("./db.json")),
      __marker_1: PhantomData,
    }
  }

  fn init(&mut self) -> Result<(), error::Error> {
    todo!();
  }

  fn get_entry(&mut self) -> Result<TEntry, error::Error> {
    todo!()
  }

  fn create_entry(&mut self, credentials: TEntry::Credentials) -> Result<TEntry, error::Error> {
    return Ok(TEntry::new(self.get_next_id(), credentials));
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
