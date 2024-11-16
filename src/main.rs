use std::{
    fs::{self},
    io::{self},
    time::{self, Instant, SystemTime},
};

mod password_generator;
mod vault;

use vault::Vault;

fn main() {
    let mut vault = Vault::new();

    if let Err(e) = vault.load_from_db() {
        eprintln!("Error: {}", e);
        return;
    }

    println!("{:?}", vault.entries_get());

    vault.entry_create(String::from("new login!"), String::from("passworkj2b34k"));

    println!("{:?}", vault.entries_get());

    vault.save_to_db();
}
