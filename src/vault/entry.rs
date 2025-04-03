use serde::{Deserialize, Serialize};

use super::crypt::{base64_to_bytes, bytes_to_base64, EncryptionPair};

#[derive(Debug)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

impl Credentials {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }

    pub fn encrypt(&self, encryption_pair: EncryptionPair) -> CredentialsEncrypted {
        let login = encryption_pair.encrypt(self.login.as_bytes()).unwrap();
        let password = encryption_pair.encrypt(self.password.as_bytes()).unwrap();

        let login_encoded = bytes_to_base64(&login);
        let password_encoded = bytes_to_base64(&password);

        return CredentialsEncrypted::new(login_encoded, password_encoded);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CredentialsEncrypted {
    pub login: String,
    pub password: String,
}

impl CredentialsEncrypted {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }

    // TODO: Вернуть Result, после рефакторинга ошибок.
    pub fn decrypt(self, encryption_pair: EncryptionPair) -> Credentials {
        let login_decoded = base64_to_bytes(self.login).unwrap();
        let login = encryption_pair.decrypt(login_decoded);

        let password_decoded = base64_to_bytes(self.password).unwrap();
        let password = encryption_pair.decrypt(password_decoded);

        return Credentials::new(
            String::from_utf8(login).unwrap(),
            String::from_utf8(password).unwrap(),
        );
    }
}

#[derive(Debug)]
pub struct Entry {
    pub id: String,
    pub credentials: Credentials,
}

impl Entry {
    pub fn new(id: String, login: String, password: String) -> Self {
        Self {
            id,
            credentials: Credentials::new(login, password),
        }
    }

    // TODO: Вернуть Result, после рефакторинга ошибок.
    pub fn encrypt(self, encryption_pair: EncryptionPair) -> EntryEncrypted {
        return EntryEncrypted {
            id: self.id,
            credentials: self.credentials.encrypt(encryption_pair),
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntryEncrypted {
    pub id: String,
    pub credentials: CredentialsEncrypted,
}

impl EntryEncrypted {
    // TODO: Вернуть Result, после рефакторинга ошибок.
    pub fn decrypt(self, encryption_pair: EncryptionPair) -> Entry {
        return Entry {
            id: self.id,
            credentials: self.credentials.decrypt(encryption_pair),
        };
    }
}
