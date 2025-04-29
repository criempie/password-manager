use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{base64string::Base64String, encryption_manager::IEncryptionManager};

#[derive(Serialize, Deserialize)]
pub struct Entry<Credentials> {
  id: String,
  credentials: Credentials,
}

#[derive(Serialize, Deserialize)]
pub struct EntryEncrypted<Settings> {
  id: String,
  settings: Settings,
  credentials: Base64String,
}

pub trait IEntry {
  type Credentials: Serialize + DeserializeOwned;

  fn new(id: String, credentials: Self::Credentials) -> Self;
  fn encrypt<TSettings, TEncryptionManager>(
    &self,
    manager: TEncryptionManager,
  ) -> EntryEncrypted<TSettings>
  where
    TSettings: Serialize + DeserializeOwned,
    TEncryptionManager: IEncryptionManager;
}

pub trait IEntryEncrypted {
  type Settings: Serialize + DeserializeOwned;

  fn new(id: String, settings: Self::Settings, credentials: Base64String) -> Self;
  fn decrypt<TCredentials, TEncryptionManager>(
    &self,
    manager: TEncryptionManager,
  ) -> Entry<TCredentials>
  where
    TCredentials: Serialize + DeserializeOwned,
    TEncryptionManager: IEncryptionManager;
}

impl<Credentials> IEntry for Entry<Credentials>
where
  Credentials: Serialize + DeserializeOwned,
{
  type Credentials = Credentials;

  fn new(id: String, credentials: Credentials) -> Self {
    Self { id, credentials }
  }

  fn encrypt<TSettings, TEncryptionManager>(
    &self,
    _manager: TEncryptionManager,
  ) -> EntryEncrypted<TSettings>
  where
    TSettings: Serialize + DeserializeOwned,
    TEncryptionManager: IEncryptionManager,
  {
    todo!();
  }
}

impl<Settings> IEntryEncrypted for EntryEncrypted<Settings>
where
  Settings: Serialize + DeserializeOwned,
{
  type Settings = Settings;

  fn new(id: String, settings: Settings, credentials: Base64String) -> Self {
    Self {
      id,
      settings,
      credentials,
    }
  }

  fn decrypt<TCredentials, TEncryptionManager>(
    &self,
    _manager: TEncryptionManager,
  ) -> Entry<TCredentials>
  where
    TCredentials: Serialize + DeserializeOwned,
    TEncryptionManager: IEncryptionManager,
  {
    todo!();
  }
}

// #[derive(Debug)]
// pub struct Entry {
//   pub id: String,
//   pub settings: EntrySettings,
//   pub credentials: Credentials,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct EntryEncrypted {
//   pub id: String,
//   pub settings: EntrySettings,
//   pub credentials: CredentialsEncrypted,
// }

// impl Entry {
//   pub fn new(id: String, login: String, password: String) -> Self {
//     Self {
//       id,
//       settings: EntrySettings {
//         iv: EncryptionIV::generate_new(),
//       },
//       credentials: Credentials::new(login, password),
//     }
//   }

//   // TODO: Вернуть Result, после рефакторинга ошибок.
//   pub fn encrypt(self, encryption_key: EncryptionKey) -> EntryEncrypted {
//     return EntryEncrypted {
//       id: self.id.clone(),
//       credentials: self.encrypt_credentials(encryption_key),
//       settings: self.settings,
//     };
//   }

//   // TODO: return Result<>.
//   pub fn encrypt_credentials(&self, encryption_key: EncryptionKey) -> CredentialsEncrypted {
//     let login = encryption_key
//       .encrypt(&self.settings.iv, self.credentials.login.as_bytes())
//       .unwrap();

//     let password = encryption_key
//       .encrypt(&self.settings.iv, self.credentials.password.as_bytes())
//       .unwrap();

//     let login_encoded = Base64String::encode(login);
//     let password_encoded = Base64String::encode(password);

//     return CredentialsEncrypted::new(login_encoded, password_encoded);
//   }
// }

// impl EntryEncrypted {
//   // TODO: Вернуть Result, после рефакторинга ошибок.
//   pub fn decrypt(self, encryption_key: EncryptionKey) -> Entry {
//     return Entry {
//       id: self.id.clone(),
//       credentials: self.decrypt_credentials(encryption_key),
//       settings: self.settings,
//     };
//   }

//   // TODO: Вернуть Result, после рефакторинга ошибок.
//   pub fn decrypt_credentials(&self, encryption_key: EncryptionKey) -> Credentials {
//     let login_decoded = self.credentials.login.decode().unwrap();
//     let login = encryption_key.decrypt(&self.settings.iv, login_decoded);

//     let password_decoded = self.credentials.password.decode().unwrap();
//     let password = encryption_key.decrypt(&self.settings.iv, password_decoded);

//     return Credentials::new(
//       String::from_utf8(login).unwrap(),
//       String::from_utf8(password).unwrap(),
//     );
//   }
// }

// #[derive(Debug)]
// pub enum Error {}

// impl std::error::Error for Error {}

// impl fmt::Display for Error {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     // match self {}
//     Ok(())
//   }
// }
