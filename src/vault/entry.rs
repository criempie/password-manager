use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Credentials {
    login: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    id: String,
    credentials: Credentials,
}
