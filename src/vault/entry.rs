use core::fmt;

use serde::{Deserialize, Serialize};

use super::crypt::{Base64String, EncryptionIV, EncryptionKey};

#[derive(Debug)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CredentialsEncrypted {
    pub login: Base64String,
    pub password: Base64String,
}

impl Credentials {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }
}

impl CredentialsEncrypted {
    pub fn new(login: Base64String, password: Base64String) -> Self {
        Self { login, password }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    iv: EncryptionIV,
}

#[derive(Debug)]
pub struct Entry {
    pub id: String,
    pub settings: Settings,
    pub credentials: Credentials,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntryEncrypted {
    pub id: String,
    pub settings: Settings,
    pub credentials: CredentialsEncrypted,
}

impl Entry {
    pub fn new(id: String, login: String, password: String) -> Self {
        Self {
            id,
            settings: Settings {
                iv: EncryptionIV::generate_new(),
            },
            credentials: Credentials::new(login, password),
        }
    }

    // TODO: Вернуть Result, после рефакторинга ошибок.
    pub fn encrypt(self, encryption_key: EncryptionKey) -> EntryEncrypted {
        return EntryEncrypted {
            id: self.id.clone(),
            credentials: self.encrypt_credentials(encryption_key),
            settings: self.settings,
        };
    }

    // TODO: return Result<>.
    pub fn encrypt_credentials(&self, encryption_key: EncryptionKey) -> CredentialsEncrypted {
        let login = encryption_key
            .encrypt(&self.settings.iv, self.credentials.login.as_bytes())
            .unwrap();

        let password = encryption_key
            .encrypt(&self.settings.iv, self.credentials.password.as_bytes())
            .unwrap();

        let login_encoded = Base64String::encode(login);
        let password_encoded = Base64String::encode(password);

        return CredentialsEncrypted::new(login_encoded, password_encoded);
    }
}

impl EntryEncrypted {
    // TODO: Вернуть Result, после рефакторинга ошибок.
    pub fn decrypt(self, encryption_key: EncryptionKey) -> Entry {
        return Entry {
            id: self.id.clone(),
            credentials: self.decrypt_credentials(encryption_key),
            settings: self.settings,
        };
    }

    // TODO: Вернуть Result, после рефакторинга ошибок.
    pub fn decrypt_credentials(&self, encryption_key: EncryptionKey) -> Credentials {
        let login_decoded = self.credentials.login.decode().unwrap();
        let login = encryption_key.decrypt(&self.settings.iv, login_decoded);

        let password_decoded = self.credentials.password.decode().unwrap();
        let password = encryption_key.decrypt(&self.settings.iv, password_decoded);

        return Credentials::new(
            String::from_utf8(login).unwrap(),
            String::from_utf8(password).unwrap(),
        );
    }
}

#[derive(Debug)]
pub enum Error {}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match self {}
        Ok(())
    }
}
