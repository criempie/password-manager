pub mod database;
pub mod error;
mod vault;

use database::Database;
use vault::{PasswordEntry, Vault_reborn};

fn main() {
    // let mut vault = Vault_reborn::new();
    // let entry = PasswordEntry::new(
    //     Vault_reborn::generate_id(),
    //     String::from("Hello world"),
    //     String::from("12345sdfkj234"),
    // );

    // vault.add_entry(entry);

    let mut db = Database::new();

    if let Err(e) = db.open() {
        println!("{}", e);
    }

    let entries = match db.get_entries() {
        Ok(entries) => entries,
        Err(e) => panic!("{}", e),
    };

    println!("Entries: {:?}", entries);
}
