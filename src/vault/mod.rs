mod database;
pub mod entries;

use std::fmt::Write;

use database::Database;
use entries::ClassicEntry;

const UNLOCK_KEY_ITERATIONS: u8 = 4;
const UNLOCK_KEY_LENGTH: u8 = 16;
const UNLOCK_KEY_SALT: &str = "TODO_123123123";
const UNLOCK_KEY_MEM_SIZE: u32 = 65535;

pub struct Vault {
    db: Database<ClassicEntry>,
}

impl Vault {
    pub fn new() -> Vault {
        return Vault {
            db: Database::new(),
        };
    }

    pub fn unlock(&mut self) {
        self.db.open().unwrap();
        self.db.sync_from_file().unwrap();
    }

    pub fn lock(&mut self) {
        self.db.sync_to_file().unwrap();
        self.db.close();
    }

    // TODO: Return reference to entry from db after db.entry_get() is implemented.
    pub fn entry_create(&mut self, login: &str, password: &str) -> ClassicEntry {
        let entry = ClassicEntry::new(login, password);

        self.db.entry_insert(entry.clone());
        self.db.sync_to_file().unwrap();

        return entry;
    }

    pub fn entries_get(&self) -> &Vec<ClassicEntry> {
        return self.db.entries_get();
    }

    // Converts the original user-friendly master password into its hash with big length.
    pub fn derive_unlock_key(password: String) -> String {
        let argon = argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(UNLOCK_KEY_MEM_SIZE, UNLOCK_KEY_ITERATIONS.into(), 4, None)
                .unwrap(),
        );

        let mut buffer = [0u8; UNLOCK_KEY_LENGTH as usize];
        argon
            .hash_password_into(password.as_bytes(), UNLOCK_KEY_SALT.as_bytes(), &mut buffer)
            .unwrap();

        return Vault::bytes_to_hex_string(&buffer);
    }

    fn bytes_to_hex_string(bytes: &[u8]) -> String {
        let mut result = String::new();

        for &byte in bytes {
            write!(&mut result, "{:02x}", byte).unwrap();
        }

        return result;
    }
}
