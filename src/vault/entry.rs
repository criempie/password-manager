use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Credentials {
    login: String,
    password: String,
}

impl Credentials {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    id: String,
    credentials: Credentials,
}

impl Entry {
    pub fn new(id: String, login: String, password: String) -> Self {
        Self {
            id,
            credentials: Credentials::new(login, password),
        }
    }
}
