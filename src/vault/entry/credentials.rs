use serde::{Deserialize, Serialize};

use crate::vault::base64string::Base64String;

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
