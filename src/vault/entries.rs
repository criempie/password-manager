use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassicEntry {
    pub id: String,
    pub login: String,
    password: String,
}

impl ClassicEntry {
    pub fn new(login: &str, password: &str) -> ClassicEntry {
        return ClassicEntry {
            id: Uuid::new_v4().as_simple().to_string(),
            login: String::from(login),
            password: String::from(password),
        };
    }
}
