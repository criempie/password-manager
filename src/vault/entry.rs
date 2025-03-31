use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

impl Credentials {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
}
