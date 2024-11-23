mod database;
pub mod entries;

use database::Database;
use entries::ClassicEntry;

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
}
